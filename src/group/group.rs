use reqwest::Method;

use crate::{client::Client, error::ClientError, request::Request};

use super::entity::{GroupDetailEntity, GroupInfoEntity, GroupListEntity, GroupMemberEntity};
use super::parameters::{GroupParameter, GroupUserRoleParameter};

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
    pub fn list_by_user(&self, user: &str) -> Result<GroupListEntity, ClientError> {
        Request::new(&self.client).send::<GroupListEntity>(
            &format!("users/{}/groups", user),
            Method::GET,
            None,
        )
    }

    ///  获取公开组织列表
    ///  
    ///  **GET:** `/groups`
    pub fn list(&self) -> Result<GroupListEntity, ClientError> {
        Request::new(&self.client).send::<GroupListEntity>("groups", Method::GET, None)
    }

    /// 创建 Group
    ///
    /// **POST:** `/groups`
    pub fn create(&self, parameters: GroupParameter) -> Result<GroupDetailEntity, ClientError> {
        Request::new(&self.client).send::<GroupDetailEntity>(
            "groups",
            Method::POST,
            Some(Box::new(parameters)),
        )
    }

    /// 获取单个组织的详细信息
    ///
    /// **GET:** `/groups/:login` `/groups/:id`
    pub fn get_sign(&self, sign: &str) -> Result<GroupDetailEntity, ClientError> {
        Request::new(&self.client).send::<GroupDetailEntity>(
            &format!("groups/{}", sign),
            Method::GET,
            None,
        )
    }

    /// 更新单个组织的详细信息
    ///
    /// **PUT:** `/groups/:login` `/groups/:id`
    pub fn update(
        &self,
        group: &str,
        parameters: GroupParameter,
    ) -> Result<GroupDetailEntity, ClientError> {
        Request::new(&self.client).send::<GroupDetailEntity>(
            &format!("groups/{}", group),
            Method::PUT,
            Some(Box::new(parameters)),
        )
    }

    /// 删除组织
    ///
    /// **DELETE:** `/groups/:login` `/groups/:id`
    pub fn delete(&self, group: &str) -> Result<GroupDetailEntity, ClientError> {
        Request::new(&self.client).send::<GroupDetailEntity>(
            &format!("groups/{}", group),
            Method::DELETE,
            None,
        )
    }

    /// 获取组织成员信息
    ///
    /// **GTE:** `/groups/:login/users` `/groups/:id/users`
    pub fn group_users(&self, group: &str) -> Result<GroupMemberEntity, ClientError> {
        Request::new(&self.client).send::<GroupMemberEntity>(
            &format!("groups/{}/users", group),
            Method::GET,
            None,
        )
    }

    /// 增加或更新组织成员
    ///
    /// **PUT:** `/groups/:group_login/users/:login` `/groups/:group_id/users/:login`
    pub fn update_user_role(
        &self,
        group: &str,
        user: &str,
        role: GroupUserRoleParameter,
    ) -> Result<GroupMemberEntity, ClientError> {
        Request::new(&self.client).send::<GroupMemberEntity>(
            &format!("groups/{}/users/{}", group, user),
            Method::PUT,
            Some(Box::new(role)),
        )
    }

    /// 删除组织成员
    ///
    /// **DELETE:** `/groups/:group_login/users/:login` `/groups/:group_id/users/:login`
    pub fn delete_user(&self, group: &str, user: &str) -> Result<GroupMemberEntity, ClientError> {
        Request::new(&self.client).send::<GroupMemberEntity>(
            &format!("groups/{}/users/{}", group, user),
            Method::DELETE,
            None,
        )
    }
}
