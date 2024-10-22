<template>
    <el-container class="main_container">
        <el-header>
            <h1>Register</h1>
        </el-header>
        <el-main>
            <el-form>
                <el-form-item label="Username">
                    <el-input v-model="username"></el-input>
                </el-form-item>
                <el-form-item label="Password">
                    <el-input v-model="password" type="password"></el-input>
                </el-form-item>
                <el-form-item label="Confirm Password">
                    <el-input v-model="confirmPassword" type="password"></el-input>
                </el-form-item>
                <el-button type="primary" @click="register">Register</el-button>
            </el-form>
        </el-main>
    </el-container>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { connectWebSocket, sendMessage, onMessage } from '@/utils/ws';

const username = ref('');
const password = ref('');
const confirmPassword = ref('');
const router = useRouter();

const showAlert = (message: string) => {
    alert(message);
};

const register = async () => {
    if (!username.value) {
        showAlert('Username cannot be empty');
        return;
    }
    if (!password.value) {
        showAlert('Password cannot be empty');
        return;
    }
    if (password.value !== confirmPassword.value) {
        showAlert('Passwords do not match');
        return;
    }

    try {
        await connectWebSocket('ws://localhost:9000');

        onMessage('RegisterResponse', (response) => {
            if (response.status === 'success') {
                showAlert('Registration successful, please log in.');
                router.push('/login');
            } else {
                showAlert(response.message);
                return;
            }
        });

        const registerData = { messageType: 'RegisterRequest', username: username.value, password: password.value };
        sendMessage(registerData);
    } catch (error) {
        console.error("Failed to connect to WebSocket:", error);
        showAlert('Failed to connect to the server. Please try again later.');
    }
};
</script>

<style></style>
