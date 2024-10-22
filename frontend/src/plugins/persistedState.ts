// src/plugins/persistedState.ts
import { PiniaPluginContext } from 'pinia'

const persistedStatePlugin = ({ store }: PiniaPluginContext) => {
  const storedState = localStorage.getItem(store.$id)
  if (storedState) {
    store.$patch(JSON.parse(storedState))
  }

  store.$subscribe((mutation, state) => {
    localStorage.setItem(store.$id, JSON.stringify(state))
  })
}

export default persistedStatePlugin