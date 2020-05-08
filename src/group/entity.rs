use crate::entity::{
    AbilitiesEntity, AbilitiesRepoEntity, AbilitiesUserEntity, UserAttributeEntity,
};
use serde::{Deserialize, Serialize};

/// 组织列表信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupListEntity {
    data: Option<Box<Vec<GroupListDataEntity>>>,
}
/// 组织列表`data`信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupListDataEntity {
    id: u32,
    login: String,
    name: String,
    avatar_url: String,
    books_count: u32,
    public_books_count: u32,
    topics_count: u32,
    public_topics_count: u32,
    members_count: u32,
    public: u32,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    _serializer: String,
}

/// 单个组织详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroupDetailEntity {
    // abilities: GroupDetailAbilitiesEntity,
    abilities: Option<AbilitiesEntity>,
    data: Option<GroupDetailDataEntity>,
    meta: Option<GroupDetailMetaEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailMetaEntity {
    topic_enable: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailDataEntity {
    id: u32,
    space_id: u32,
    organization_id: u32,
    login: String,
    name: String,
    avatar_url: String,
    owner_id: u32,
    books_count: u32,
    public_books_count: u32,
    topics_count: u32,
    public_topics_count: u32,
    members_count: u32,
    public: u32,
    description: String,
    created_at: String,
    updated_at: String,
    _serializer: String,
}

// 组织成员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberEntity {
    data: Option<Box<GroupMemberDataEntity>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberDataEntity {
    id: u32,
    group_id: u32,
    user_id: u32,
    role: u32,
    status: u32,
    group: Option<String>,
    created_at: String,
    updated_at: String,
    _serializer: String,
    user: Option<UserAttributeEntity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInfoEntity {
    data: Option<Box<GroupDetailDataEntity>>,
}
