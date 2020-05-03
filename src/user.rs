use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{Client, Request};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserEntity {
    data: Box<UserEntityInner>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserEntityInner {
    id: u32,
    #[serde(rename = "type")]
    _type: String,
    space_id: u32,
    account_id: u32,
    login: String,
    name: String,
    avatar_url: String,
    books_count: u32,
    public_books_count: u32,
    followers_count: u32,
    following_count: u32,
    public: u32,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    _serializer: String,
}

#[derive(Debug, Clone)]
pub struct User {
    client: Client,
}

impl User {
    pub fn new(client: Client) -> User {
        User { client }
    }

    ///  获取单个用户信息
    ///
    ///  **GET:** `/users/:login` `/users/:id`
    pub fn get(&self) -> UserEntity {
        let rt = Request::new(&self.client)
            .send("user", Method::GET, None)
            .expect("HHHHHHHHHHH");
        serde_json::from_str::<UserEntity>(&rt).unwrap()
    }

    ///  获取认证的用户的个人信息
    ///
    ///  **GET:** `/user`
    pub fn get_sign<T>(&self, sign: T) -> UserEntity
    where
        T: Into<String>,
    {
        let rt = Request::new(&self.client)
            .send(&format!("users/{}", sign.into()), Method::GET, None)
            .expect("Get_Sign ERROR!!!!!!!!!");
        serde_json::from_str::<UserEntity>(&rt).unwrap()
    }
}
