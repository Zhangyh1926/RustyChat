<template>
  <el-container class="main-container">
    <el-header v-if="!isNotHeader">
      <el-menu class="menu" :router="true" mode="horizontal" :default-active="activeIndex" :ellipsis="false">
        <el-menu-item index="/">Chatting</el-menu-item>
        <el-menu-item index="/friends">Friends Center</el-menu-item>
        <el-sub-menu index="2">
          <template #title>Workspace</template>
          <el-menu-item index="2-1">item one</el-menu-item>
          <el-menu-item index="2-2">item two</el-menu-item>
          <el-menu-item index="2-3">item three</el-menu-item>
          <el-sub-menu index="2-4">
            <template #title>item four</template>
            <el-menu-item index="2-4-1">item one</el-menu-item>
            <el-menu-item index="2-4-2">item two</el-menu-item>
            <el-menu-item index="2-4-3">item three</el-menu-item>
          </el-sub-menu>
        </el-sub-menu>

        <!-- 头像和个人信息 -->
        <el-menu-item index="profile">
          <el-popover
            placement="bottom"
            width="200"
            trigger="hover"
            content-class="profile-popover"
          >
            <template #reference>
              <el-avatar size="small" src="your-avatar-url.jpg"></el-avatar>
            </template>
              <div class="profile-info">
                <p><strong>用户名:</strong> {{ userName }}</p>
                <p><strong>朋友数目:</strong> {{ friendCount }}</p>
                <p><strong>签名:</strong> {{ signature }}</p>
                <el-link href="/profile-center" type="primary">个人中心</el-link>
              </div>
          </el-popover>
        </el-menu-item>

        <el-menu-item index="/login" onclick="">Logout</el-menu-item>
      </el-menu>
    </el-header>
    <el-container>
      <router-view></router-view>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute } from 'vue-router';

const activeIndex = ref('/');
const route = useRoute();
const isNotHeader = computed(() => route.path === '/login' || route.path === '/register');

// 假设用户信息来自API或者本地状态
const userName = ref('John Doe');
const friendCount = ref(10);
const signature = ref('Life is short, live it well.');
</script>

<style scoped>
.avatar-large {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  margin-bottom: 10px;
}

.profile-dropdown {
  width: 200px;
}

.profile-info p {
  margin: 0;
  line-height: 1.5;
}

.menu > .el-menu-item:nth-child(1) {
  margin-right: 20px;
}

.menu > .el-sub-menu:nth-child(3) {
  margin-right: auto;
}
</style>
