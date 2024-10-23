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

macro_rules! response_with_message {
    (
        $( $name:ident { $( $field:ident : $type:ty ),* $(,)? } ),* $(,)? 
    ) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(tag = "messageType")]
        pub enum Response {
            $(
                $name {
                    status: String,
                    message: String,
                    $( $field: $type ),* // 允许没有字段
                }
            ),*
        }

        impl Response {
            pub fn new(name: &'static str, status: String, message: String) -> Self {
                match name {
                    $( stringify!($name) => {
                        Response::$name {
                            status,
                            message,
                            $( $field: None ),* // 允许没有字段
                        }
                    }, )*
                    _ => panic!("Unknown response type"),
                }
            }
        }
    };
}

// 定义响应类型
response_with_message! {
    GenericResponse {},
    LoginResponse {
        id: Option<i32>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        access_token_expire: Option<SystemTime>,
        refresh_token_expire: Option<SystemTime>,
    },
    FriendListResponse {
        friends: Option<Vec<FriendListItem>>,
    },
    MessageListResponse {
        messages: Option<Vec<MessageListItem>>,
    },
    MessagePushResponse {},
    RegisterResponse {
        id: Option<i32>,
    },
    FriendCenterResponse {
        friends: Option<Vec<FriendCenterItem>>,
    },
    HeartbeatResponse {},
    RefreshTokenResponse {
        access_token: Option<String>,
        refresh_token: Option<String>,
        access_token_expire: Option<SystemTime>,
        refresh_token_expire: Option<SystemTime>,
    },
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