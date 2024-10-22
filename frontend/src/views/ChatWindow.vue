<-- src/views/ChatWindow.vue -->
  <template>
    <el-container>
      <el-main class="chat-window">
        <el-scrollbar style="display: flex; flex-direction: column; z-index: 0;">
          <div v-if="selectedFriend">
            <div class="chat-header">
              <h2>{{ selectedFriend.friend_name }}</h2>
            </div>
            <div class="messages">
              <div v-for="(message, index) in messages" :key="index"
                :class="['message', message.sender_id === props.selectedFriend?.friend_id ? 'received' : 'sent']">
                <div class="message-bubble">
                  <span>{{ message.message }}</span>
                  <div class="timestamp">{{ formatLastMessageTime(convertEpochToDate(message.send_time)) }}</div>
                </div>
              </div>

            </div>
          </div>
          <div v-else class="no-friend-prompt">
            <p>Please select a friend to chat with.</p>
          </div>
        </el-scrollbar>
      </el-main>
      <el-footer v-if="selectedFriend" style="display: flex; flex-direction: column-reverse; z-index: 10;">
        <div class="input-area" style="display: flex; align-items: center;">
          <el-input class="input-box" v-model="newMessage" placeholder="Type a message" show-word-limit maxlength="800"
            type="textarea" :autosize="{ minRows: 2 }" input-style="border-radius: 20px;" resize="none" />
          <el-button @click="sendInputMessage" :disabled="!newMessage.trim()"
            :style="{ backgroundColor: newMessage.trim() ? 'green' : 'gray' }" style="margin-left: 10px;">
            Send
          </el-button>
        </div>
      </el-footer>
    </el-container>
  </template>

  <script lang="ts" setup>
  import { ref, onMounted, watch } from 'vue';
  import type { FriendInList, MessageListResponse } from '../utils/types';
  import { connectWebSocket, sendMessage, onMessage } from '@/utils/ws';
  import { showAlert } from '@/utils/alert';
  import { Message } from '../utils/types';
  import { loginStateStore } from '@/stores/loginState';
  import { formatLastMessageTime } from './timeFormatter';
  import { convertEpochToDate } from '@/utils/convertEpochToDate';
  import { MessagePushResponse } from '../utils/types';

  const props = defineProps<{ userid: number; selectedFriend: FriendInList | null }>();
  const newMessage = ref('');
  const loginState = loginStateStore();

  const messages = ref<Message[]>([]);

  watch(
    () => props.selectedFriend,
    async (newValue, oldValue) => {
      if (newValue) {
        try {
          await connectWebSocket('ws://localhost:9000');

          const messageListData = { messageType: 'MessageListRequest', my_userid: props.userid, other_userid: newValue.friend_id };
          sendMessage(messageListData);
          console.log('Friend list requested');
        } catch (error) {
          console.error("Failed to connect to WebSocket:", error);
          showAlert('Failed to connect to the server. Please try again later.');
        }
      } else {
        console.log('朋友取消选择');
      }
    }
  );

  const sendInputMessage = async () => {
    // 检查输入的有效性
    if (!props.userid) {
      showAlert('Userid cannot be empty, bug occurs');
      return;
    }
    if (!props.selectedFriend) {
      showAlert('Selected friend cannot be empty');
      return;
    }

    try {
      await connectWebSocket('ws://localhost:9000');

      const loginData = { messageType: 'MessagePushRequest', sender_id: props.userid, receiver_id: props.selectedFriend.friend_id, message: newMessage.value };
      sendMessage(loginData);
    } catch (error) {
      console.error("Failed to connect to WebSocket:", error);
      showAlert('Failed to connect to the server. Please try again later.');
    }
  }

  onMounted(() => {
    const handleMessagePushResponse = (response: MessagePushResponse) => {
      if (response.status === 'success') {
        const messageListData = { messageType: 'MessageListRequest', my_userid: props.userid, other_userid: props.selectedFriend?.friend_id };
        sendMessage(messageListData);
      } else {
        showAlert(response.message);  // 处理错误消息
        return;
      }
    }

    const handleMessageListResponse = (response: MessageListResponse) => {
      if (response.status === 'success') {
        messages.value = response.messages;
        console.log('Message list received:', response.messages);
      } else {
        showAlert(response.message);
        return;
      }
    }

    onMessage('MessageListResponse', handleMessageListResponse);
    onMessage('MessagePushResponse', handleMessagePushResponse);
  });

</script>

  <style scoped>
  .chat-window {
    padding: 10px;
    max-height: calc(100vh - 120px);
    display: flex;
    flex-direction: column;
  }

  .chat-header {
    background-color: #f3f3f3;
    padding: 10px;
    border-bottom: 1px solid #e0e0e0;
    position: sticky;
    /* 固定在顶部 */
    top: 0;
    z-index: 1;
  }

  .messages {
    flex-grow: 1;
    /* 允许消息列表占据剩余空间 */
    overflow-y: auto;
    /* 添加垂直滚动 */
    padding: 10px;
  }

  .message {
    margin-bottom: 10px;
    display: flex;
  }

  .message.sent {
    justify-content: flex-end;
  }

  .message.received {
    justify-content: flex-start;
  }

  .message-bubble {
    max-width: 60%;
    padding: 10px;
    border-radius: 15px;
    background-color: #e0f7fa;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    text-align: justify;
  }

  .message.sent .message-bubble {
    background-color: #c8e6c9;
  }

  .timestamp {
    font-size: 0.8em;
    color: #888;
    margin-top: 5px;
    text-align: right;
  }

  .input-area {
    padding: 10px;
    border-top: 1px solid #e0e0e0;
    background-color: #fff;
  }

  .input-box {
    display: flex;
    flex-direction: column-reverse;
    max-height: 200px;
  }

  .no-friend-prompt {
    padding: 20px;
    text-align: center;
    color: #888;
  }
</style>
