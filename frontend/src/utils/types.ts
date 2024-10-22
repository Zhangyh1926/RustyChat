// src/views/types.ts
export interface Message {
  id: number;
  sender_id: number;
  receiver_id: number;
  message: string;
  send_time: SystemTime;
}

// 通用的好友信息
export interface FriendBase {
  friend_id: number;
  friend_name: string;
  avatar_url: string | null;
}

// 用于聊天好友列表的好友信息
export interface FriendInList extends FriendBase {
  latest_message: string;
  last_message_time: SystemTime;
}

// 用于好友中心列表的好友信息
export interface FriendInCenter extends FriendBase {
  signature: string | null;
}

export type MessagePushResponse = {
  status: string;
  message: string;
  timestamp?: SystemTime;
  id?: number;
}

export type MessageListResponse = {
  status: string;
  message: string;
  messages: Message[];
}

export type SystemTime = { secs_since_epoch: number; nanos_since_epoch: number };