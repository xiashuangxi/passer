use crate::{AbilitiesEntity, UserAttributeEntity};
use serde::{Deserialize, Serialize};

// 文档列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocListEntity {
    data: Option<Box<Vec<DocListDataEntity>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocListDataEntity {
    id: u32,
    slug: String,
    title: String,
    description: Option<String>,
    user_id: u32,
    book_id: u32,
    format: String,
    public: u32,
    status: u32,
    view_status: u32,
    read_status: u32,
    likes_count: u32,
    comments_count: u32,
    content_updated_at: String,
    created_at: String,
    updated_at: String,
    published_at: String,
    first_published_at: String,
    draft_version: u32,
    last_editor_id: u32,
    word_count: u32,
    cover: String,
    custom_description: Option<String>,
    last_editor: Option<UserAttributeEntity>,
    book: Option<String>,
    _serializer: String,
}

/// 文档详细信息
pub struct DocDetailEntity {
    abilities: Option<AbilitiesEntity>,
    data: Option<DocDetailDataEntity>,
}

pub struct DocDetailDataEntity {
    id: u32,
    slug: String,
    title: String,
    book_id: u32,
    book: Option<DocDetailDataBookEntity>,
    user_id: u32,
    creator: Option<UserAttributeEntity>,

    format: String,
    body: String,
    body_draft: String,
    body_html: String,
    body_lake: String,
    body_draft_lake: String,
    public: u32,
    status: Option<String>,
    view_status: u32,
    read_status: u32,
    likes_count: u32,
    comments_count: u32,
    content_updated_at: String,
    deleted_at: Option<String>,
    created_at: String,
    updated_at: String,
    published_at: String,
    first_published_at: String,
    word_count: u32,
    cover: String,
    description: Option<String>,
    custom_description: Option<String>,
    _serializer: String,
}

pub struct DocDetailDataBookEntity {
    id: u32,
    _type: String,
    slug: String,
    name: String,
    user_id: u32,
    description: Option<String>,
    creator_id: u32,
    public: u32,
    items_count: u32,
    likes_count: u32,
    watches_count: u32,
    content_updated_at: String,
    updated_at: String,
    created_at: String,
    namespace: String,
    _serializer: String,
    user: Option<UserAttributeEntity>,
}
