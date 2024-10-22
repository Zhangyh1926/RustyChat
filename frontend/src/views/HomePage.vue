<-- src/views/HomePage.vue -->
  <template>
    <FriendList :userid=userid :onSelect=handleFriendSelect />
    <ChatWindow :userid=userid :selectedFriend="selectedFriend" />
  </template>

  <script lang="ts" setup>
  import { ref } from 'vue';

  import FriendList from './FriendsList.vue';
  import ChatWindow from './ChatWindow.vue';

  import type { FriendInList } from '../utils/types';
  const selectedFriend = ref<FriendInList | null>(null);
  import { loginStateStore } from '@/stores/loginState';

  const loginState = loginStateStore();

  const userid = ref(loginState.id);

  const handleFriendSelect = (friend: FriendInList) => {
    console.log('Friend selected:', friend);
    console.log('selectedFriend:', selectedFriend);

    selectedFriend.value = friend;
  };
</script>
  <style>

  html,
  body {
    margin: 0;
    padding: 0;
  }

  .main-container {
    height: 100vh;
  }

  .el-header,
  .el-footer {
    background-color: #b3c0d1;
    color: var(--el-text-color-primary);
    text-align: center;
    line-height: 60px;
    padding: 0%;
  }

  .el-aside {
    background-color: #d3dce6;
    color: var(--el-text-color-primary);
    text-align: center;
  }

  .el-main {
    background-color: #e9eef3;
    color: var(--el-text-color-primary);
    text-align: center;
  }

  .el-container.main {
    min-height: 100vh;
    flex-direction: column;
  }

  .el-container.content-container {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
  }

  .el-menu--horizontal>.el-menu-item:nth-child(1) {
    margin-right: auto;
  }
</style>