use crate::{AbilitiesEntity, UserAttributeEntity};
use serde::{Deserialize, Serialize};

// 文档列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocListEntity {
    pub data: Option<Box<Vec<DocListDataEntity>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocListDataEntity {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub user_id: u32,
    pub book_id: u32,
    pub format: String,
    pub public: u32,
    pub status: u32,
    pub view_status: u32,
    pub read_status: u32,
    pub likes_count: u32,
    pub comments_count: u32,
    pub content_updated_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
    pub first_published_at: String,
    pub draft_version: u32,
    pub last_editor_id: u32,
    pub word_count: u32,
    pub cover: String,
    pub custom_description: Option<String>,
    pub last_editor: Option<UserAttributeEntity>,
    pub book: Option<String>,
    pub _serializer: String,
}

/// 文档详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocDetailEntity {
    pub abilities: Option<AbilitiesEntity>,
    pub data: Option<DocDetailDataEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocDetailDataEntity {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub book_id: u32,
    pub book: Option<DocDetailDataBookEntity>,
    pub user_id: u32,
    pub creator: Option<UserAttributeEntity>,

    pub format: String,
    pub body: String,
    pub body_draft: String,
    pub body_html: String,
    pub body_lake: String,
    pub body_draft_lake: String,
    pub public: u32,
    pub status: Option<String>,
    pub view_status: u32,
    pub read_status: u32,
    pub likes_count: u32,
    pub comments_count: u32,
    pub content_updated_at: String,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
    pub first_published_at: String,
    pub word_count: u32,
    pub cover: String,
    pub description: Option<String>,
    pub custom_description: Option<String>,
    pub _serializer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocDetailDataBookEntity {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub slug: String,
    pub name: String,
    pub user_id: u32,
    pub description: Option<String>,
    pub creator_id: u32,
    pub public: u32,
    pub items_count: u32,
    pub likes_count: u32,
    pub watches_count: u32,
    pub content_updated_at: String,
    pub updated_at: String,
    pub created_at: String,
    pub namespace: String,
    pub _serializer: String,
    pub user: Option<UserAttributeEntity>,
}
