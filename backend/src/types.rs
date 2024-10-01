// src/types.rs

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "messageType")]
pub enum Request {
    LoginRequest { username: String, password: String },
    MessageListRequest { my_userid: i32, other_userid: i32 },
    FriendListRequest { userid: i32 },
    MessagePushRequest { sender_id: i32, receiver_id: i32, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "messageType")]
pub enum Response {
    LoginResponse {
        status: String,
        message: String,
        id: Option<i32>,
    },
    FriendListResponse {
        status: String,
        message: String,
        friends: Option<Vec<FriendListItem>>,
    },
    MessageListResponse {
        status: String,
        message: String,
        messages: Option<Vec<MessageListItem>>,
    },
    MessagePushResponse {
        status: String,
        message: String,
        timestamp: Option<SystemTime>, // 返回消息的时间戳
        id: Option<i32>, // 返回消息的id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendListItem {
    pub friend_id: i32,
    pub friend_name: String,
    pub latest_message: String,
    pub last_message_time: SystemTime,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageListItem {
    pub message_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub send_time: SystemTime,
}