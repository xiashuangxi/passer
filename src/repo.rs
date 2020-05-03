use crate::{
    Client, CreateRepoParameter, ListRepoParameter, RepoTypeParameter, Request, UpdateRepoParameter,
};
use reqwest::Method;

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
    pub fn list_for_user<T>(&self, user: T, parameter: ListRepoParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("users/{}/repos", user.into()),
            Method::GET,
            Some(Box::new(parameter)),
        );
    }

    /// 获取某个团队的知识库列表
    ///
    /// **GET:** `/groups/:login/repos` `/groups/:id/repos`
    pub fn list_for_group<T>(&self, group: T, parameter: ListRepoParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("groups/{}/repos", group.into()),
            Method::GET,
            Some(Box::new(parameter)),
        );
    }

    /// 在团队里创建知识库
    ///
    /// **POST:** `/groups/:login/repos` `/groups/:id/repos`
    pub fn create_for_group<T>(&self, group: T, parameter: CreateRepoParameter)
    where
        T: Into<String>,
    {
        // TODO: 未完成创建
        Request::new(&self.client).send(
            &format!("groups/{}/repos", group.into()),
            Method::POST,
            Some(Box::new(parameter)),
        );
    }

    /// 在用户下创建知识库
    ///
    /// **POST:** `/users/:login/repos` `/users/:id/repos`
    pub fn create_for_user<T>(&self, user: T, parameter: CreateRepoParameter)
    where
        T: Into<String>,
    {
        // TODO:
        Request::new(&self.client).send(
            &format!("users/{}/repos", user.into()),
            Method::POST,
            Some(Box::new(parameter)),
        );
    }

    /// 获取知识库详细
    ///
    /// **GET:** `/repos/:namespace` `/repos/:id`
    pub fn get<T>(&self, repo: T, parameter: RepoTypeParameter)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(
            &format!("repos/{}", repo.into()),
            Method::GET,
            Some(Box::new(parameter)),
        );
    }

    /// 更新知识库信息
    ///
    /// **PUT:** `/repos/:namespace` `/repos/:id`
    pub fn update<T>(&self, repo: T, parameter: UpdateRepoParameter)
    where
        T: Into<String>,
    {
        // TODO:
        Request::new(&self.client).send(
            &format!("repos/{}", repo.into()),
            Method::PUT,
            Some(Box::new(parameter)),
        );
    }

    /// 删除知识加
    ///
    /// **DELETE:** `/repos/:namespace` `/repos/:id`
    pub fn delete<T>(&self, repo: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(&format!("repos/{}", repo.into()), Method::DELETE, None);
    }

    /// 获取一个知识库的目录结构
    ///
    /// **GET:** `/repos/:namespace/toc` `/repos/:id/toc`
    pub fn toc<T>(&self, repo: T)
    where
        T: Into<String>,
    {
        Request::new(&self.client).send(&format!("repos/{}/toc", repo.into()), Method::GET, None);
    }
}
