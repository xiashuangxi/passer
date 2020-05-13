use crate::{doc::Doc, group::Group, repo::Repo, user::User};

/// 默认 `API` 地址。
const API_BASE_URL: &str = "https://www.yuque.com/api";
/// `API` 版本。
const API_VERSION: &str = "v2";
/// 默认 `USER-AGENT` 值。
const API_USER: &str = "@Passer/SDK";

/// `API` 请求头
#[derive(Debug, Clone)]
pub struct Header {
    inner: Box<Vec<HeaderInner>>,
}

#[derive(Debug, Clone)]
pub struct HeaderInner(String, String);

impl Default for Header {
    fn default() -> Header {
        Header::new("CONTENT-TYPE", "application/json").add_header("USER-AGENT", API_USER)
    }
}

impl Header {
    pub fn new<S>(name: S, value: S) -> Header
    where
        S: Into<String>,
    {
        Header {
            inner: Box::new(vec![HeaderInner::new(name, value)]),
        }
    }

    pub fn add_header<S>(mut self, name: S, value: S) -> Header
    where
        S: Into<String>,
    {
        self.inner.push(HeaderInner::new(name, value));
        self
    }

    pub fn inner(&self) -> Vec<HeaderInner> {
        self.inner.to_vec()
    }
}

impl HeaderInner {
    pub fn new<S>(name: S, value: S) -> HeaderInner
    where
        S: Into<String>,
    {
        HeaderInner(name.into(), value.into())
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn value(&self) -> &str {
        self.1.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct Home {
    inner: Box<HomeInner>,
}

#[derive(Debug, Clone)]
pub struct HomeInner {
    pub host: String,
    pub version: String,
}

impl Default for Home {
    fn default() -> Home {
        Home::new()
    }
}

impl Home {
    pub fn new() -> Home {
        Home::custom(API_BASE_URL, API_VERSION)
    }

    pub fn custom<S>(host: S, version: S) -> Home
    where
        S: Into<String>,
    {
        Home {
            inner: Box::new(HomeInner {
                host: host.into(),
                version: version.into(),
            }),
        }
    }

    pub fn host(&self) -> &str {
        &self.inner.host
    }

    pub fn version(&self) -> &str {
        &self.inner.version
    }
}

impl ToString for Home {
    fn to_string(&self) -> String {
        format!("{}/{}", self.host(), self.version())
    }
}

/// `API` 处理客户端
#[derive(Debug, Clone)]
pub struct Client {
    pub header: Header,
    pub home: Home,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client::custom(token, API_BASE_URL, API_USER)
    }

    pub fn custom(token: &str, url: &str, user: &str) -> Client {
        Client {
            header: Header::default()
                .add_header("X-Auth-Token", token)
                .add_header("USER-AGENT", user),
            home: Home::custom(url, API_VERSION),
        }
    }
    pub fn header(mut self, header: Header) -> Client {
        self.header = header;
        self
    }

    pub fn add_header<S>(mut self, name: S, value: S) -> Client
    where
        S: Into<String>,
    {
        // FIXME: 这里比较疑惑可能需要进行优化。#以后再进行处理
        self.header = self.header.clone().add_header(name, value);
        self
    }

    pub fn get_user(self) -> User {
        User::new(self)
    }

    pub fn get_group(self) -> Group {
        Group::new(self)
    }

    pub fn get_repo(self) -> Repo {
        Repo::new(self)
    }

    pub fn get_doc(self) -> Doc {
        Doc::new(self)
    }
}
