use crate::entity::{
    AbilitiesEntity, AbilitiesRepoEntity, AbilitiesUserEntity, UserAttributeEntity,
};
use serde::{Deserialize, Serialize};

/// 组织列表信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupListEntity {
    pub data: Option<Box<Vec<GroupListDataEntity>>>,
}
/// 组织列表`data`信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupListDataEntity {
    pub id: u32,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub books_count: u32,
    pub public_books_count: u32,
    pub topics_count: u32,
    pub public_topics_count: u32,
    pub members_count: u32,
    pub public: u32,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
}

/// 单个组织详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroupDetailEntity {
    // abilities: GroupDetailAbilitiesEntity,
    pub abilities: Option<AbilitiesEntity>,
    pub data: Option<GroupDetailDataEntity>,
    pub meta: Option<GroupDetailMetaEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailMetaEntity {
    pub topic_enable: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailDataEntity {
    pub id: u32,
    pub space_id: u32,
    pub organization_id: u32,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub owner_id: u32,
    pub books_count: u32,
    pub public_books_count: u32,
    pub topics_count: u32,
    pub public_topics_count: u32,
    pub members_count: u32,
    pub public: u32,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
}

// 组织成员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroupMemberEntity {
    pub data: Option<Box<Vec<GroupMemberDataEntity>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroupMemberDataEntity {
    pub id: u32,
    pub group_id: u32,
    pub user_id: u32,
    pub role: u32,
    pub status: u32,
    pub group: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
    pub user: Option<UserAttributeEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInfoEntity {
    pub data: Option<Box<GroupDetailDataEntity>>,
}
