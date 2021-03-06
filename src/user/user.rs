use reqwest::Method;

use super::entity::UserEntity;
use crate::{client::Client, error::ClientError, request::Request};

/// 处理用户 `API`
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
    pub fn get(&self) -> Result<UserEntity, ClientError> {
        Request::new(&self.client).send::<UserEntity>("user", Method::GET, None)
    }

    ///  获取认证的用户的个人信息
    ///
    ///  **GET:** `/user`
    pub fn get_sign(&self, sign: &str) -> Result<UserEntity, ClientError> {
        Request::new(&self.client).send::<UserEntity>(&format!("users/{}", sign), Method::GET, None)
    }
}
