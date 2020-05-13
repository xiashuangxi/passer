use crate::AbilitiesEntity;
use serde::{Deserialize, Serialize};

/// 知识库列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoListEntity {
    pub data: Option<Box<Vec<RepoListDataEntity>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoListDataEntity {
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
    pub user: Option<RepoUserEntity>,
    pub _serializer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoUserEntity {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub login: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: String,
    pub books_count: u32,
    pub public_books_count: u32,
    pub followers_count: u32,
    pub following_count: u32,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
}

/// 知识库详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDetailEntity {
    // abilities: RepoDetailAbilitiesEntity,
    pub abilities: Option<AbilitiesEntity>,
    pub data: Option<RepoDetailDataEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDetailDataEntity {
    pub id: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub slug: String,
    pub name: String,
    pub user_id: u32,
    pub description: Option<String>,
    pub toc: String,
    pub toc_yml: String,
    pub creator_id: u32,
    pub public: u32,
    pub items_count: u32,
    pub likes_count: u32,
    pub watches_count: u32,
    pub pinned_at: Option<String>,
    pub archived_at: Option<String>,
    pub namespace: String,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
    pub user: Option<RepoUserEntity>,
}

/// 知识库目录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TocEntity {
    pub data: Option<Vec<TocDataEntity>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TocDataEntity {
    #[serde(rename = "type")]
    pub _type: String,
    pub title: String,
    pub level: u32,
    pub url: String,
    pub uuid: String,
    pub id: u32,
    pub depth: u32,
    pub slug: String,
}
