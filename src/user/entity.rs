/// 用户信息。
///
/// ## `Json` 结构
/// ```json
/// {
///     "data": {
///         ...
///     }
/// }
/// ```
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserEntity {
    pub data: Option<Box<UserEntityData>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserEntityData {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub space_id: u32,
    pub account_id: u32,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub books_count: u32,
    pub public_books_count: u32,
    pub followers_count: u32,
    pub following_count: u32,
    pub public: u32,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
}
