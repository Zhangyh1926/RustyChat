import { createApp } from 'vue';
import App from './App.vue';
import './index.css';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import router from './router';
import { createPinia } from 'pinia';
import persistedStatePlugin from './plugins/persistedState';
import { onMessage } from './utils/ws';
import { loginStateStore } from './stores/loginState';
import { showAlert } from './utils/alert';

const pinia = createPinia();
pinia.use(persistedStatePlugin);
const app = createApp(App);

app.use(pinia);
app.use(ElementPlus);
app.use(router);
app.mount('#root');

let loginState = loginStateStore();
onMessage('GenericResponse', (response) => { // 监听 GenericResponse 消息
    if (response.status === 'success') {
        ;
    } else {
        showAlert(response.message);
        return;
    }
});
