use crate::{Client, DocDetailParameter, DocParameter, Request, UpdateDocParameter};
use reqwest::Method;

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
    pub fn list_for_repo<T>(&self, repo: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(&format!("repos/{}/docs", repo.into()), Method::GET, None);
    }

    /// 获取单篇文档的详细信息
    ///
    /// **GET:** `/repos/:namespace/docs/:slug`
    pub fn get<T>(&self, namespace: T, slug: T, parameter: DocDetailParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("repos/{}/docs/{}", namespace.into(), slug.into()),
            Method::GET,
            Some(Box::new(parameter)),
        );
    }

    /// 创建文档
    ///
    /// **POST:** `/repos/:namespace/docs` `/repos/:id/docs`
    pub fn create<T>(&self, repo: T, parameter: DocParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("repos/{}/docs", repo.into()),
            Method::POST,
            Some(Box::new(parameter)),
        );
    }

    /// 更新文档
    ///
    /// **PUT:** `/repos/:namespace/docs/:id` `/repos/:repo_id/docs/:id`
    pub fn update<T>(&self, repo: T, id: T, parameter: UpdateDocParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("repos/{}/docs/{}", repo.into(), id.into()),
            Method::PUT,
            Some(Box::new(parameter)),
        );
    }

    /// 删除文档
    ///
    /// **DELETE:** `/repos/:namespace/docs/:id` `/repos/:repo_id/docs/:id`
    pub fn delete<T>(&self, repo: T, id: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("repos/{}/docs/{}", repo.into(), id.into()),
            Method::DELETE,
            None,
        );
    }
}
