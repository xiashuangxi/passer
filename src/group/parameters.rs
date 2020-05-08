use crate::Parameter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GroupParameter {
    name: String,
    login: String,
    description: String,
}

impl GroupParameter {
    pub fn new<T>(name: T, login: T, description: T) -> GroupParameter
    where
        T: Into<String>,
    {
        GroupParameter {
            name: name.into(),
            login: login.into(),
            description: description.into(),
        }
    }
}

impl Parameter for GroupParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("name", self.name.clone());
        hm.insert("login", self.login.clone());
        hm.insert("description", self.description.clone());
        hm
    }
}

#[derive(Debug)]
pub struct GroupUserRoleParameter {
    role: u32,
}

impl Default for GroupUserRoleParameter {
    fn default() -> GroupUserRoleParameter {
        GroupUserRoleParameter { role: 1 }
    }
}
impl GroupUserRoleParameter {
    pub fn new(role: u32) -> GroupUserRoleParameter {
        GroupUserRoleParameter { role }
    }
}

impl Parameter for GroupUserRoleParameter {
    fn inner(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::<&str, String>::new();
        hm.insert("role", self.role.to_string());
        hm
    }
}
