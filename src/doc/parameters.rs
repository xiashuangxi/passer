use crate::Parameter;
use std::collections::HashMap;

pub struct DocDetailParameter {
    raw: u32,
}

impl Default for DocDetailParameter {
    fn default() -> DocDetailParameter {
        DocDetailParameter { raw: 1 }
    }
}

impl DocDetailParameter {
    pub fn new(raw: u32) -> DocDetailParameter {
        DocDetailParameter { raw }
    }
}

impl Parameter for DocDetailParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("raw", self.raw.to_string());
        hm
    }
}

pub struct DocParameter {
    title: String,
    slug: String,
    public: u32,
    format: String,
    body: String,
}

impl DocParameter {
    pub fn new<T>(title: T, slug: T, public: u32, format: T, body: T) -> DocParameter
    where
        T: Into<String>,
    {
        DocParameter {
            title: title.into(),
            slug: slug.into(),
            public,
            format: format.into(),
            body: body.into(),
        }
    }
}

impl Parameter for DocParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("title", self.title.clone());
        hm.insert("slug", self.slug.clone());
        hm.insert("public", self.public.to_string());
        hm.insert("format", self.format.clone());
        hm.insert("body", self.body.clone());
        hm
    }
}

pub struct UpdateDocParameter {
    title: String,
    slug: String,
    public: u32,
    body: String,
}

impl UpdateDocParameter {
    pub fn new<T>(title: T, slug: T, public: u32, body: T) -> UpdateDocParameter
    where
        T: Into<String>,
    {
        UpdateDocParameter {
            title: title.into(),
            slug: slug.into(),
            public,
            body: body.into(),
        }
    }
}

impl Parameter for UpdateDocParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("title", self.title.clone());
        hm.insert("slug", self.slug.clone());
        hm.insert("public", self.public.to_string());
        hm.insert("body", self.body.clone());
        hm
    }
}
