<template>
  <el-container>

    <el-main>
      <el-tabs v-model="activeTab">
        <!-- 好友列表 -->
        <el-tab-pane label="好友列表" name="friends">
          <el-card>
            <el-table v-if="friends.length !== 0" :data="friends" style="width: 100%">
              <el-table-column prop="avatar_url" label="头像" width="80">
                <template #default="scope">
                  <el-avatar :src="scope.row.avatar" class="avatar" />
                </template>
              </el-table-column>

              <el-table-column prop="friend_name" label="用户名" width="200" />

              <el-table-column prop="signature" label="个性签名" />

              <el-table-column label="操作" width="120">
                <template #default="scope">
                  <el-button type="danger" size="small" @click="removeFriend(scope.row.id)">删除</el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-empty v-if="friends.length === 0" description="暂无好友" />
          </el-card>
        </el-tab-pane>

        <!-- 好友申请 -->
        <el-tab-pane label="好友申请" name="requests">
          <el-card>
            <el-table v-if="friendRequests.length !== 0" :data="friendRequests" style="width: 100%">
              <el-table-column prop="avatar" label="头像" width="80">
                <template #default="scope">
                  <el-avatar :src="scope.row.avatar" class="avatar" />
                </template>
              </el-table-column>

              <el-table-column prop="name" label="用户名" width="200" />

              <el-table-column prop="signature" label="个性签名" />

              <el-table-column label="操作" width="160">
                <template #default="scope">
                  <el-button type="primary" size="small" @click="acceptRequest(scope.row.id)">接受</el-button>
                  <el-button type="danger" size="small" @click="declineRequest(scope.row.id)">拒绝</el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-empty v-if="friendRequests.length === 0" description="暂无好友申请" />
          </el-card>
        </el-tab-pane>

        <!-- 添加好友 -->
        <el-tab-pane label="添加好友" name="addFriend">
          <el-card>
            <el-input v-model="newFriend" placeholder="输入好友名称" clearable />
            <el-button type="success" @click="addFriend" style="margin-top: 10px">添加好友</el-button>
          </el-card>
        </el-tab-pane>
      </el-tabs>
    </el-main>
  </el-container>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import type { FriendInCenter } from '../utils/types';
import unimplemented from 'ts-unimplemented'
import { connectWebSocket, onMessage, sendMessage } from '@/utils/ws';
import { showAlert } from '@/utils/alert';
import { loginStateStore } from '@/stores/loginState';

const loginState = loginStateStore();

const userid = ref(loginState.id);

const activeTab = ref<string>('friends');
const friends = ref<FriendInCenter[]>([]);

const friendRequests = ref<FriendInCenter[]>([
  { friend_id: 3, friend_name: '王五', avatar_url: 'https://via.placeholder.com/40', signature: '一切皆有可能' },
]);

onMounted(async () => {
    try {
      await connectWebSocket('ws://localhost:9000');

      onMessage('FriendCenterResponse', (response) => {
        if (response.status === 'success') {
          friends.value = response.friends;
        } else {
          showAlert(response.message);
          return;
        }
      });

      const friendCenterData = { messageType: 'FriendCenterRequest', userid: userid.value };
      sendMessage(friendCenterData);
      console.log('Friend center requested');
    } catch (error) {
      console.error("Failed to connect to WebSocket:", error);
      showAlert('Failed to connect to the server. Please try again later.');
    }
  });

const newFriend = ref<string>('');

// 删除好友
const removeFriend = (id: number) => {
  return unimplemented();
};

// 接受好友请求
const acceptRequest = (id: number) => {
  return unimplemented();
};

// 拒绝好友请求
const declineRequest = (id: number) => {
  return unimplemented();
};

// 添加好友
const addFriend = () => {
  return unimplemented();
};
</script>

<style scoped>
.el-header {
  text-align: center;
  background-color: #f5f5f5;
  padding: 10px;
  border-bottom: 1px solid #eaeaea;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
}

.el-card {
  margin: 20px;
}

.el-tabs__header {
  justify-content: center;
}

.el-main {
  padding: 20px;
}
</style>
