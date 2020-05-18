use crate::{Client, ClientError, Request};
use reqwest::Method;

use super::entity::DocListEntity;
use super::parameters::*;

#[derive(Debug, Clone)]
pub struct Doc {
    client: Client,
}

impl Doc {
    pub fn new(client: Client) -> Doc {
        Doc { client }
    }

    /// 获取一个仓库的文档列表
    ///
    /// **GET:** `/repos/:namespace/docs` `/repos/:id/docs`
    pub fn list_for_repo(&self, repo: &str) -> Result<DocListEntity, ClientError> {
        Request::new(&self.client).send::<DocListEntity>(
            &format!("repos/{}/docs", repo),
            Method::GET,
            None,
        )
    }

    /// 获取单篇文档的详细信息
    ///
    /// **GET:** `/repos/:namespace/docs/:slug`
    pub fn get(
        &self,
        namespace: &str,
        slug: &str,
        parameter: DocDetailParameter,
    ) -> Result<DocListEntity, ClientError> {
        Request::new(&self.client).send::<DocListEntity>(
            &format!("repos/{}/docs/{}", namespace, slug),
            Method::GET,
            Some(Box::new(parameter)),
        )
    }

    /// 创建文档
    ///
    /// **POST:** `/repos/:namespace/docs` `/repos/:id/docs`
    pub fn create(
        &self,
        repo: &str,
        parameter: DocParameter,
    ) -> Result<DocListEntity, ClientError> {
        Request::new(&self.client).send::<DocListEntity>(
            &format!("repos/{}/docs", repo),
            Method::POST,
            Some(Box::new(parameter)),
        )
    }

    /// 更新文档
    ///
    /// **PUT:** `/repos/:namespace/docs/:id` `/repos/:repo_id/docs/:id`
    pub fn update(
        &self,
        repo: &str,
        id: &str,
        parameter: UpdateDocParameter,
    ) -> Result<DocListEntity, ClientError> {
        Request::new(&self.client).send::<DocListEntity>(
            &format!("repos/{}/docs/{}", repo, id),
            Method::PUT,
            Some(Box::new(parameter)),
        )
    }

    /// 删除文档
    ///
    /// **DELETE:** `/repos/:namespace/docs/:id` `/repos/:repo_id/docs/:id`
    pub fn delete(&self, repo: &str, id: &str) -> Result<DocListEntity, ClientError> {
        Request::new(&self.client).send::<DocListEntity>(
            &format!("repos/{}/docs/{}", repo, id),
            Method::DELETE,
            None,
        )
    }
}
