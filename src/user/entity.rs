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
        data: Option<Box<UserEntityData>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct UserEntityData {
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

