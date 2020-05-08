use crate::AbilitiesEntity;
use serde::{Deserialize, Serialize};

/// 知识库列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoListEntity {
    data: Option<Box<Vec<RepoListDataEntity>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoListDataEntity {
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
    user: Option<RepoUserEntity>,
    _serializer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoUserEntity {
    id: u32,
    _type: String,
    login: String,
    name: String,
    description: Option<String>,
    avatar_url: String,
    books_count: u32,
    public_books_count: u32,
    followers_count: u32,
    following_count: u32,
    created_at: String,
    updated_at: String,
    _serializer: String,
}

/// 知识库详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDetailEntity {
    // abilities: RepoDetailAbilitiesEntity,
    abilities: Option<AbilitiesEntity>,
    data: Option<RepoDetailDataEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDetailDataEntity {
    id: u32,
    _type: String,
    slug: String,
    name: String,
    user_id: u32,
    description: Option<String>,
    toc: String,
    toc_yml: String,
    creator_id: u32,
    public: u32,
    items_count: u32,
    likes_count: u32,
    watches_count: u32,
    pinned_at: Option<String>,
    archived_at: Option<String>,
    namespace: String,
    created_at: String,
    updated_at: String,
    _serializer: String,
    user: Option<RepoUserEntity>,
}

/// 知识库目录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TocEntity {
    data: Option<Vec<TocDataEntity>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TocDataEntity {
    _type: String,
    title: String,
    level: u32,
    url: String,
    uuid: String,
    id: u32,
    depth: u32,
    slug: String,
}
