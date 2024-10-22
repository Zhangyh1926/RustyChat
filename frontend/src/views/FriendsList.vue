<-- src/views/FriendList.vue -->
  <template>
    <el-aside width="300px" class="aside">
      <div class="friend-list-header">
        <el-button @click="sortByTime">Sort by Time</el-button>
        <el-button @click="sortByName">Sort by Name</el-button>
      </div>
      <el-scrollbar class="friend-list" height="400px">
        <div v-for="friend in sortedFriends" :key="friend.friend_id" class="friend-item" @click="selectFriend(friend)"
          :class="{ 'selected-friend': selectedFriend && selectedFriend.friend_id === friend.friend_id }">

          <img :src="friend.avatar_url || defaultAvatarUrl" class="friend-avatar" alt="avatar" />
          <div class="friend-info">
            <div class="friend-name">{{ friend.friend_name }}</div>
            <div class="friend-message">{{ friend.latest_message }}</div>
            <div class="last-message-time">{{ formatLastMessageTime(convertEpochToDate(friend.last_message_time)) }}</div>
          </div>
        </div>
      </el-scrollbar>
    </el-aside>
  </template>


  <script lang="ts" setup>
  import { ref, onMounted } from 'vue';
  import type { FriendInList } from '../utils/types';
  import { formatLastMessageTime } from './timeFormatter'; // 导入时间格式化函数
  import { convertEpochToDate } from '@/utils/convertEpochToDate';
  import { connectWebSocket, sendMessage, onMessage } from '@/utils/ws';
  import { showAlert } from '@/utils/alert';

  const friends = ref<FriendInList[]>([]);
  const sortedFriends = ref<FriendInList[]>([]);

  const props = defineProps<{ userid: number; onSelect: (friend: FriendInList) => void; }>();

  const defaultAvatarUrl: string = 'https://via.placeholder.com/40';

  onMounted(async () => {
    try {
      await connectWebSocket('ws://localhost:9000');

      onMessage('FriendListResponse', (response) => {
        if (response.status === 'success') {
          friends.value = response.friends;
          sortedFriends.value = [...friends.value];
        } else {
          showAlert(response.message);
          return;
        }
      });

      const friendListData = { messageType: 'FriendListRequest', userid: props.userid };
      sendMessage(friendListData);
      console.log('Friend list requested');
    } catch (error) {
      console.error("Failed to connect to WebSocket:", error);
      showAlert('Failed to connect to the server. Please try again later.');
    }
  });

  const selectedFriend = ref<FriendInList | null>(null);

  // Sorting functions
  const sortByTime = () => {
    if (friends.value.length > 0) {
      sortedFriends.value = [...friends.value].sort((a, b) => convertEpochToDate(b.last_message_time).getTime() - convertEpochToDate(a.last_message_time).getTime());
    }
  };

  const sortByName = () => {
    if (friends.value.length > 0) {
      sortedFriends.value = [...friends.value].sort((a, b) => a.friend_name.localeCompare(b.friend_name));
    }
  };

  // Select a friend
  const selectFriend = (friend: FriendInList) => {
    props.onSelect(friend);
  };
</script>

  <style scoped>
  .aside {
    background-color: #f3f3f3;
    padding: 10px;
  }

  .friend-list {
    padding: 10px;
    height: 400px;
  }

  .friend-item {
    display: flex;
    padding: 10px;
    align-items: center;
    border-bottom: 1px solid #dcdfe6;
    cursor: pointer;
    transition: background-color 0.3s;
    /* 添加过渡效果 */
  }

  .friend-item.selected-friend {
    background-color: #d1e0f0 !important;
    /* 选中时的背景色 */
  }

  .friend-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    margin-right: 10px;
  }

  .friend-info {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .friend-name {
    font-weight: bold;
    text-align: left;
  }

  .friend-message {
    font-size: 12px;
    color: #909399;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 1;
    line-clamp: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: left;
  }

  .last-message-time {
    font-size: 12px;
    color: #909399;
    text-align: right;
  }
</style>
