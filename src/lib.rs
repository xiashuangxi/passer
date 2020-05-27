//! [`yuque`] 平台 `API` 的 `rust` 实现。
//!
//! [`yuque`]: https://www.yuque.com/
//!
//! ## 示例
//!```no_run
//! use passer::client::Client;
//! Client::new("xxxxxxxxxxxxxxxxxxxxxxxxx")
//!            .get_user()
//!            .get();
//!```
//!
//! `Client` 需要提供一个 `Token`。

pub mod client;
pub mod error;
pub mod request;

pub mod doc;
pub mod group;
pub mod repo;
pub mod user;

// pub use client::{Client, Header, HeaderInner, Home};
// pub use error::{ClientError, InternalError, RequestError};
// pub use request::Request;

use entity::*;
use parameters::Parameter;

/// 参数
pub mod parameters {

    use std::collections::HashMap;

    /// 参数的通用方法，可以返回一个`HashMap<&str, String>`结构
    pub trait Parameter {
        fn inner(&self) -> HashMap<&str, String>;
    }
}

/// 实体
pub mod entity {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct AbilitiesEntity {
        pub read: bool,
        pub update: bool,
        pub destroy: bool,

        // group
        pub group_user: Option<AbilitiesUserEntity>,
        pub repo: Option<AbilitiesRepoEntity>,

        // repo
        pub doc: Option<AbilitiesDocEntity>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct AbilitiesUserEntity {
        pub create: bool,
        pub update: bool,
        pub destroy: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct AbilitiesRepoEntity {
        pub create: bool,
        pub update: bool,
        pub destroy: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct AbilitiesDocEntity {
        pub create: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct UserAttributeEntity {
        pub id: Option<u32>,
        #[serde(rename = "type")]
        pub _type: String,
        pub login: String,
        pub name: String,
        pub description: Option<String>,
        pub avatar_url: String,

        pub books_count: Option<u32>,
        pub public_books_count: Option<u32>,

        pub followers_count: Option<u32>,
        pub following_count: Option<u32>,
        pub created_at: String,
        pub updated_at: String,
        pub _serializer: String,
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let user = client::Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH").get_user().get().unwrap();
		println!("{:?}", user);
		client::Client::new("sfasdf").get_user().get().unwrap();
    }
}