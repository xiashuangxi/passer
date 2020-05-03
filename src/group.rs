use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{Client, GroupParameter, GroupUserRoleParameter, Request};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupListEntity {
    data: Box<Vec<GroupEntity>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupEntity {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailEntity {
    abilities: GroupDetailAbilitiesEntity,
    meta: GroupDetailMetaEntity,
    data: GroupDetailInner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailAbilitiesEntity {
    read: bool,
    update: bool,
    destory: bool,

    group_user: AbilitiesGroupUser,
    repo: AbilitiesRepo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilitiesGroupUser {
    create: bool,
    update: bool,
    destroy: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilitiesRepo {
    create: bool,
    update: bool,
    destory: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailMetaEntity {
    topic_enable: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetailInner {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupUserEntity {
    id: u32,
    group_id: u32,
    user_id: u32,
    role: u32,
    status: u32,
    group: Option<String>,
    created_at: String,
    updated_at: String,
    _serializer: String,
    user: GroupUserItemEntity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupUserItemEntity {
    id: u32,
    utype: String,
    login: String,
    name: String,
    description: Option<String>,
    avatar_url: String,
    followers_count: u32,
    following_count: u32,
    created_at: String,
    updated_at: String,
    _serializer: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    client: Client,
}

impl Group {
    pub fn new(client: Client) -> Group {
        Group { client }
    }

    ///  获取某个用户的加入的组织列表
    ///
    /// **GET:** `/users/:login/groups` `/users/:id/groups`
    pub fn list_by_user<T>(&self, user: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("users/{}/groups", user.into()),
            Method::GET,
            None,
        );
    }

    ///  获取公开组织列表
    ///  
    ///  **GET:** `/groups`
    pub fn list(&self) {
        Request::new(&self.client).send("/groups", Method::GET, None);
    }

    /// 创建 Group
    ///
    /// **POST:** `/groups`
    pub fn create(&self, parameters: GroupParameter) {
        Request::new(&self.client).send(
            &format!("groups"),
            Method::POST,
            Some(Box::new(parameters)),
        );
    }

    /// 获取单个组织的详细信息
    ///
    /// **GET:** `/groups/:login` `/groups/:id`
    pub fn get_sign<T>(&self, sign: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(&format!("groups/{}", sign.into()), Method::GET, None);
    }

    /// 更新单个组织的详细信息
    ///
    /// **PUT:** `/groups/:login` `/groups/:id`
    pub fn update<T>(&self, group: T, parameters: GroupParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("/groups/{}", group.into()),
            Method::PUT,
            Some(Box::new(parameters)),
        );
    }

    /// 删除组织
    ///
    /// **DELETE:** `/groups/:login` `/groups/:id`
    pub fn delete<T>(&self, group: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(&format!("group/{}", group.into()), Method::DELETE, None);
    }

    /// 获取组织成员信息
    ///
    /// **GTE:** `/groups/:login/users` `/groups/:id/users`
    pub fn group_users<T>(&self, group: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("groups/{}/users", group.into()),
            Method::GET,
            None,
        );
    }

    /// 增加或更新组织成员
    ///
    /// **PUT:** `/groups/:group_login/users/:login` `/groups/:group_id/users/:login`
    pub fn update_user_role<T>(&self, group: T, user: T, role: GroupUserRoleParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("groups/{}/users/{}", group.into(), user.into()),
            Method::PUT,
            Some(Box::new(role)),
        );
    }

    /// 删除组织成员
    ///
    /// **DELETE:** `/groups/:group_login/users/:login` `/groups/:group_id/users/:login`
    pub fn delete_user<T>(&self, group: T, user: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("groups/{}/users/{}", group.into(), user.into()),
            Method::DELETE,
            None,
        );
    }
}
