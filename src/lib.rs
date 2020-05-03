mod client;
mod doc;
mod error;
mod group;
mod parameters;
mod repo;
mod request;
mod result;
mod user;

pub use client::{Client, Header, HeaderInner, Home};
pub use error::Result;
pub use group::Group;
pub use parameters::{
    CreateRepoParameter, DocDetailParameter, DocParameter, GroupParameter, GroupUserRoleParameter,
    ListRepoParameter, Parameter, RepoTypeParameter, UpdateDocParameter, UpdateRepoParameter,
};
pub use request::Request;
pub use result::ResultEntity;
pub use user::User;

fn print_type_name_of<T>(_: T) {
    println!("{}", unsafe { std::any::type_name::<T>() })
}

#[cfg(test)]
mod tests {

    use crate::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct Param {
        name: String,
        value: String,
    }
    // #[test]
    // fn test_request() {
    //     use std::collections::HashMap;
    //     let mut map = HashMap::new();
    //     map.insert("name", "rust");
    //     map.insert("login", "xiashddsduangxi");
    //     map.insert("description", "dfsdfa");
    //     let req = request::Request::new(&Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH"))
    //         .send("groups", reqwest::Method::POST, Some(map));
    //     println!("{:?}", req);
    // }

    // #[test]
    // fn test_user() {
    //     // let c = Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH");
    //     // let u = c.get_user().get();
    //     // println!("{:?}", c);
    //     // println!("{:?}", u);

    //     let c = Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH").add_header("fasd", "fasdf");

    //     let c = Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH")
    //         .get_user()
    //         .get();
    //     let c = Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH")
    //         .get_user()
    //         .get_sign("538911");
    //     println!("##### Result User: {:?}", &c);
    // }

    #[test]
    fn test_group_crate() {
        let ss = GroupParameter::new("jjjjjj", "fasdfas", "fasdfa");
        let c = Client::new("UbOInTQWrGr8uzKGYXJgxMTq0GIMbNnVnjFwUqRH")
            .get_group()
            .create(ss);
        println!("##### Result User: {:?}", &c);
    }
}
