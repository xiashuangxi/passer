use crate::Parameter;
use std::collections::HashMap;

pub struct ListRepoParameter {
    /// `Book`, `Design`, `all`
    _type: String,
    /// 用于分页
    offset: u32,
}

impl Default for ListRepoParameter {
    fn default() -> ListRepoParameter {
        ListRepoParameter {
            _type: "Book".to_string(),
            offset: u32::max_value(),
        }
    }
}

impl ListRepoParameter {
    pub fn new<T>(_type: T, offset: u32) -> ListRepoParameter
    where
        T: Into<String>,
    {
        ListRepoParameter {
            _type: _type.into(),
            offset,
        }
    }
}

impl Parameter for ListRepoParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("type", self._type.clone());
        hm.insert("offset", self.offset.to_string());
        hm
    }
}

pub struct CreateRepoParameter {
    /// 仓库名
    name: String,
    slug: String,
    description: Option<String>,
    /// - 0：私密
    /// - 1：所有人可见
    /// - 2：空间成员可见
    /// - 3：空间所有人（包含外部联系人）可见
    /// - 4：知识库成员可见
    public: u32,
    /// `Book`,`Design`
    _type: String,
}

impl CreateRepoParameter {
    pub fn new<T>(
        name: T,
        slug: T,
        description: Option<String>,
        public: u32,
        _type: T,
    ) -> CreateRepoParameter
    where
        T: Into<String>,
    {
        CreateRepoParameter {
            name: name.into(),
            slug: slug.into(),
            description: description,
            public: public,
            _type: _type.into(),
        }
    }
}

impl Parameter for CreateRepoParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("name", self.name.clone());
        hm.insert("slug", self.slug.clone());
        hm.insert("description", self.description.as_ref().unwrap().clone());
        hm.insert("public", self.public.to_string());
        hm.insert("type", self._type.to_string());
        hm
    }
}

pub struct RepoTypeParameter {
    _type: String,
}

impl Default for RepoTypeParameter {
    fn default() -> RepoTypeParameter {
        RepoTypeParameter {
            _type: "Book".to_string(),
        }
    }
}

impl RepoTypeParameter {
    pub fn new<T>(_type: T) -> RepoTypeParameter
    where
        T: Into<String>,
    {
        RepoTypeParameter {
            _type: _type.into(),
        }
    }
}
impl Parameter for RepoTypeParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("type", self._type.clone());
        hm
    }
}

pub struct UpdateRepoParameter {
    name: String,
    slug: String,
    toc: String,
    description: Option<String>,
    public: u32,
}

impl UpdateRepoParameter {
    pub fn new<T>(
        name: T,
        slug: T,
        toc: T,
        description: Option<String>,
        public: u32,
    ) -> UpdateRepoParameter
    where
        T: Into<String>,
    {
        UpdateRepoParameter {
            name: name.into(),
            slug: slug.into(),
            toc: toc.into(),
            description,
            public,
        }
    }
}

impl Parameter for UpdateRepoParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("name", self.name.clone());
        hm.insert("slug", self.slug.clone());
        hm.insert("description", self.description.as_ref().unwrap().clone());
        hm.insert("public", self.public.to_string());
        hm
    }
}
