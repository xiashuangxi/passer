//! [`yuque`] 平台 `API` 的 `rust` 实现。
//!
//! [`yuque`]: https://www.yuque.com/
//!
//! ## 示例
//!```no_run
//! use passer::Client;
//! Client::new("xxxxxxxxxxxxxxxxxxxxxxxxx")
//!            .get_user()
//!            .get();
//!```
//!
//! `Client` 需要提供一个 `Token`。

mod client;
mod error;
mod request;

pub mod doc;
pub mod group;
pub mod repo;
pub mod user;

pub use client::{Client, Header, HeaderInner, Home};
pub use error::{ClientError, InternalError, RequestError};
pub use request::Request;

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
        read: bool,
        update: bool,
        destroy: bool,

        // group
        group_user: Option<AbilitiesUserEntity>,
        repo: Option<AbilitiesRepoEntity>,

        // repo
        doc: Option<AbilitiesDocEntity>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AbilitiesUserEntity {
        create: bool,
        update: bool,
        destroy: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AbilitiesRepoEntity {
        create: bool,
        update: bool,
        destroy: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AbilitiesDocEntity {
        create: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct UserAttributeEntity {
        id: u32,
        _type: String,
        login: String,
        name: String,
        description: Option<String>,
        avatar_url: String,

        books_count: u32,
        public_books_count: u32,

        followers_count: u32,
        following_count: u32,
        created_at: String,
        updated_at: String,
        _serializer: String,
    }
}
