// src/stores/loginState.ts
import { SystemTime } from '@/utils/types';
import { defineStore } from 'pinia'

export const loginStateStore = defineStore('loginState', {
  state: () => {
    return {
      logined: false,
      id: 0,
      access_token: '',
      refresh_token: '',
      access_token_expire: { secs_since_epoch: 0, nanos_since_epoch: 0 },
      refresh_token_expire: { secs_since_epoch: 0, nanos_since_epoch: 0 }
    }
  },
  actions: {
    login(id: number, access_token: string, refresh_token: string, access_token_expire: SystemTime, refresh_token_expire: SystemTime) {
      this.$patch({
        logined: true,
        id: id,
        access_token: access_token,
        refresh_token: refresh_token,
        access_token_expire: access_token_expire,
        refresh_token_expire: refresh_token_expire
      });
    },
    logout() {
      this.$patch({
        logined: false,
        id: 0,
        access_token: '',
        refresh_token: '',
        access_token_expire: { secs_since_epoch: 0, nanos_since_epoch: 0 },
        refresh_token_expire: { secs_since_epoch: 0, nanos_since_epoch: 0 }
      });
    },
    setTokens(access_token: string, refresh_token: string, access_token_expire: SystemTime, refresh_token_expire: SystemTime) {
      this.$patch({
        access_token: access_token,
        refresh_token: refresh_token,
        access_token_expire: access_token_expire,
        refresh_token_expire: refresh_token_expire
      });
    }
  }
});
