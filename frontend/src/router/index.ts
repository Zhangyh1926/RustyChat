// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '@/views/HomePage.vue';
import LoginPage from '@/views/LoginPage.vue';
import RegisterPage from '@/views/RegisterPage.vue';
import FriendsCenter from '@/views/FriendsCenter.vue';
import { loginStateStore } from '@/stores/loginState';

const routes = [
  { path: '/', name: 'Home', component: HomePage },
  { path: '/login', name: 'Login', component: LoginPage },
  { path: '/register', name:'Register', component: RegisterPage }, 
  { path: '/friends', name:'Friends', component: FriendsCenter }, 
  { path: '/:pathMatch(.*)*', redirect: '/' },  // 重定向到主页
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// 使用 router.beforeEach 钩子来检查登录状态
router.beforeEach((to, from, next) => {
  const loginState = loginStateStore(); // 确保 Pinia 已被初始化后调用

  console.log('Navigating to:', to.path);  // Debug log
  console.log('User logged in:', loginState.logined);  // Check the login state

  // 如果用户未登录且试图访问非登录页面/注册页面，则跳转到登录页面
  if (loginState.logined === false && to.path !== '/login' && to.path !== '/register') {
    next('/login');
  } else {
    next();  // 继续导航
  }
});

export default router;
