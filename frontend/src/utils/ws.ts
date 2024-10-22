import { loginStateStore } from "@/stores/loginState";
import { he, ro } from "element-plus/es/locales.mjs";
import { convertEpochToDate } from "./convertEpochToDate";
import { useRouter } from "vue-router";
import { showAlert } from "./alert";
import { getActivePinia } from "pinia";

// src/utils/ws.ts
let ws: WebSocket | null = null;
const listeners: { [key: string]: Function[] } = {};

const heartbeatInterval = 30000; // 心跳间隔, 30s
let heartbeatTimer: number | null = null; // 心跳定时器

const router = useRouter();

export const connectWebSocket = (url: string): Promise<void> => {
    return new Promise((resolve, reject) => {
        try {
            ws = new WebSocket(url);
            
            ws.onopen = () => {
                console.log("WebSocket connected");
                startHeartbeat();
                resolve();
            };

            ws.onerror = (error) => {
                console.error("WebSocket error:", error);
                reject(new Error("WebSocket connection failed")); // 连接失败的错误
            };

            ws.onclose = (event) => {
                stopHeartbeat();
                if (event.wasClean) {
                    console.log(`Connection closed cleanly, code=${event.code}, reason=${event.reason}`);
                } else {
                    console.error('Connection closed unexpectedly');
                    reject(new Error("WebSocket connection closed unexpectedly")); // 非正常关闭的错误
                }
            };

            ws.onmessage = (event: MessageEvent) => {
                try {
                    const response = JSON.parse(event.data);
                    notifyListeners(response);
                    console.log("Received message:", response);
                } catch (parseError) {
                    console.error("Failed to parse message:", parseError);
                    reject(new Error("Failed to parse incoming message")); // 解析错误
                }
            };
        } catch (e) {
            console.error("WebSocket initialization error:", e);
            reject(new Error("WebSocket initialization failed")); // 初始化错误
        }
    });
};

export const sendMessage = (message: any, resended: boolean = false) => {
    if (!ws) {
        console.error("WebSocket is not initialized");
        return new Error("WebSocket is not initialized");
    }

    if (ws.readyState !== WebSocket.OPEN) {
        console.error("WebSocket is not open");
        return new Error("WebSocket is not open");
    }
    let loginState = loginStateStore();

    // 如果是登录，不需要检查 token
    // 刷新 token 时，也不需要检查 token
    if (message.messageType !== "LoginRequest" && message.messageType !== "RefreshTokenRequest") {
        let now = new Date();

        // 如果 refresh_token 过期，重新登录
        let refresh_token_expire = convertEpochToDate(loginState.refresh_token_expire);
        if (now > refresh_token_expire) {
            console.error("Refresh token expired");
            router.push("/login");
        }

        // 如果 access_token 过期，刷新 token
        let access_token_expire = convertEpochToDate(loginState.access_token_expire);
        if (now > access_token_expire) {
            if (resended) {
                console.error("Access token expired");
                router.push("/login");
            }

            console.error("Access token expired");
            onMessage('RefreshTokenResponse', (response) => { // 监听刷新 token 的响应
                if (response.status === 'success') {
                    loginState.setTokens(response.access_token, response.refresh_token, response.access_token_expire, response.refresh_token_expire);
                    console.log("Token refreshed");
                } else {
                    showAlert(response.message);
                    return;
                }
            });
            // 让出控制权，等待5s后再回来
            setTimeout(() => {
                sendMessage({ messageType: "RefreshTokenRequest", refresh_token: loginState.refresh_token }, true);
            }, 5000);
            return;
        }
    }

    // 通过校验，发送消息
    // 把user_id和access_token加入到消息中
    message.user_id_for_check = loginState.id;
    message.access_token_for_check = loginState.access_token;
    try {
        ws.send(JSON.stringify(message));
    } catch (sendError) {
        console.error("Failed to send message:", sendError);
        throw new Error("Failed to send message"); // 发送错误
    }
};

export const onMessage = (eventType: string, callback: (data: any) => void) => {
    if (!listeners[eventType]) {
        listeners[eventType] = [];
    }
    listeners[eventType].push(callback);
};

const notifyListeners = (data: any) => {
    const eventType = data.messageType;
    if (listeners[eventType]) {
        listeners[eventType].forEach(callback => callback(data));
    }
};

const startHeartbeat = () => {
    heartbeatTimer = window.setInterval(() => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            sendMessage({ messageType: "heartbeat" });
        };
        console.log("Heartbeat sent");
    }, heartbeatInterval);
}

const stopHeartbeat = () => {
    if (heartbeatTimer) {
        clearInterval(heartbeatTimer);
        heartbeatTimer = null;
    }
}
