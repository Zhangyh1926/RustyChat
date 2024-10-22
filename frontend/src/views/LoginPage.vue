<-- src/views/LoginPage.vue -->
<template>
    <el-container class="main_container">
        <el-header class="header">
            <h1>Login</h1>
        </el-header>
        <el-main>
            <el-form>
                <el-form-item label="Username">
                    <el-input v-model="username"></el-input>
                </el-form-item>
                <el-form-item label="Password">
                    <el-input v-model="password" type="password"></el-input>
                </el-form-item>
                <el-button type="success" @click="login">Login</el-button>
                <el-button type="info" @click="goToRegister">Register</el-button>
            </el-form>
        </el-main>
    </el-container>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { loginStateStore } from '@/stores/loginState';
import { connectWebSocket, sendMessage, onMessage } from '@/utils/ws';

const username = ref('');
const password = ref('');
const router = useRouter();
const loginState = loginStateStore();

loginState.logout(); // 清空登录状态

const showAlert = (message: string) => {
    alert(message);
};

const login = async () => {
    if (!username.value) {
        showAlert('Username cannot be empty');
        return;
    }
    if (!password.value) {
        showAlert('Password cannot be empty');
        return;
    }

    try {
        await connectWebSocket('ws://localhost:9000');

        onMessage('LoginResponse', (response) => {
            if (response.status === 'success') {
                loginState.login(response.id, response.access_token, response.refresh_token, response.access_token_expire, response.refresh_token_expire);
                router.push('/');
            } else {
                showAlert(response.message);
                return;
            }
        });

        const loginData = { messageType: 'LoginRequest', username: username.value, password: password.value };
        sendMessage(loginData);
    } catch (error) {
        console.error("Failed to connect to WebSocket:", error);
        showAlert('Failed to connect to the server. Please try again later.');
    }
};

const goToRegister = () => {
    router.push('/register'); // 跳转到注册页面
};
</script>

<style>
.header {
    height: 80px;
}
</style>
