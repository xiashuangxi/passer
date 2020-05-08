use crate::{Client, ClientError, Request};
use reqwest::Method;

use super::parameters::{
    CreateRepoParameter, ListRepoParameter, RepoTypeParameter, UpdateRepoParameter,
};

use super::entity::{RepoDetailEntity, RepoListEntity, TocEntity};

//！处理资料库 `API`
#[derive(Debug, Clone)]
pub struct Repo {
    client: Client,
}

// TODO: 没有搜索
impl Repo {
    pub fn new(client: Client) -> Repo {
        Repo { client }
    }

    /// 获取某个用户的知识库列表
    ///
    /// **GET:** `/users/:login/repos` `/users/:id/repos`
    ///
    /// Result: RepoListEntity
    pub fn list_for_user(
        &self,
        user: &str,
        parameters: ListRepoParameter,
    ) -> Result<RepoListEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("users/{}/repos", user),
            Method::GET,
            Some(Box::new(parameters)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoListEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 获取某个团队的知识库列表
    ///
    /// **GET:** `/groups/:login/repos` `/groups/:id/repos`
    pub fn list_for_group(
        &self,
        group: &str,
        parameter: ListRepoParameter,
    ) -> Result<RepoListEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("groups/{}/repos", group),
            Method::GET,
            Some(Box::new(parameter)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoListEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 在团队里创建知识库
    ///
    /// **POST:** `/groups/:login/repos` `/groups/:id/repos`
    pub fn create_for_group(
        &self,
        group: &str,
        parameter: CreateRepoParameter,
    ) -> Result<RepoDetailEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("groups/{}/repos", group),
            Method::POST,
            Some(Box::new(parameter)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoDetailEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 在用户下创建知识库
    ///
    /// **POST:** `/users/:login/repos` `/users/:id/repos`
    pub fn create_for_user(
        &self,
        user: &str,
        parameter: CreateRepoParameter,
    ) -> Result<RepoDetailEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("users/{}/repos", user),
            Method::POST,
            Some(Box::new(parameter)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoDetailEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 获取知识库详细
    ///
    /// **GET:** `/repos/:namespace` `/repos/:id`
    pub fn get(
        &self,
        repo: &str,
        parameter: RepoTypeParameter,
    ) -> Result<RepoDetailEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("repos/{}", repo),
            Method::GET,
            Some(Box::new(parameter)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoDetailEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 更新知识库信息
    ///
    /// **PUT:** `/repos/:namespace` `/repos/:id`
    pub fn update(
        &self,
        repo: &str,
        parameter: UpdateRepoParameter,
    ) -> Result<RepoDetailEntity, ClientError> {
        match Request::new(&self.client).send(
            &format!("repos/{}", repo),
            Method::PUT,
            Some(Box::new(parameter)),
        ) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoDetailEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 删除知识库
    ///
    /// **DELETE:** `/repos/:namespace` `/repos/:id`
    pub fn delete(&self, repo: &str) -> Result<RepoDetailEntity, ClientError> {
        match Request::new(&self.client).send(&format!("repos/{}", repo), Method::DELETE, None) {
            Ok(ref json) => Ok(serde_json::from_str::<RepoDetailEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 获取一个知识库的目录结构
    ///
    /// **GET:** `/repos/:namespace/toc` `/repos/:id/toc`
    pub fn toc(&self, repo: &str) -> Result<TocEntity, ClientError> {
        match Request::new(&self.client).send(&format!("repos/{}/toc", repo), Method::GET, None) {
            Ok(ref json) => Ok(serde_json::from_str::<TocEntity>(json).unwrap()),
            Err(e) => Err(e),
        }
    }
}
