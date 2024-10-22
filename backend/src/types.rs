// src/types.rs

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

macro_rules! request_with_auth {
    (
        $( $name:ident { $( $field:ident : $type:ty ),* } ),* $(,)?
    ) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(tag = "messageType")]
        pub enum Request {
            $(
                $name {
                    user_id_for_check: i32,
                    access_token_for_check: String,
                    $( $field: $type ),*
                }
            ),*
        }
    };
}

// 使用宏定义不同的请求类型
request_with_auth! {
    LoginRequest { username: String, password: String },
    MessageListRequest { other_userid: i32 },
    FriendListRequest { },
    MessagePushRequest { receiver_id: i32, message: String },
    RegisterRequest { username: String, password: String },
    FriendCenterRequest { },
    HeartbeatRequest { },
    RefreshTokenRequest { refresh_token: String },
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "messageType")]
pub enum Response {
    GenericResponse {
        status: String,
        message: String,
    },
    LoginResponse {
        status: String,
        message: String,
        id: Option<i32>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        access_token_expire: Option<SystemTime>,
        refresh_token_expire: Option<SystemTime>,
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
    },
    RegisterResponse {
        status: String,
        message: String,
        id: Option<i32>,
    },
    FriendCenterResponse {
        status: String,
        message: String,
        friends: Option<Vec<FriendCenterItem>>,
    },
    HeartbeatResponse {
        status: String,
        message: String,
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
pub struct FriendCenterItem {
    pub friend_id: i32,
    pub friend_name: String,
    pub avatar_url: Option<String>,
    pub signature: Option<String>,
    pub status: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageListItem {
    pub message_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub send_time: SystemTime,
}