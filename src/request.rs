use reqwest::{self, blocking};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{Client, Parameter, ResultEntity};

#[derive(Debug)]
pub struct Request<'a>(&'a Client);

impl<'a> Request<'a> {
    pub fn new(client: &'a Client) -> Request<'a> {
        Request(client)
    }

    pub fn send(
        &self,
        url: &str,
        method: reqwest::Method,
        parameters: Option<Box<dyn Parameter>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!(
            "@Parse::Request: {:?}",
            &format!("{}/{}", &self.0.home.to_string(), url)
        );
        let mut req = blocking::Client::new()
            .request(method, &format!("{}/{}", &self.0.home.to_string(), url));
        for x in self.0.header.inner().iter() {
            req = req.header(x.name(), x.value());
        }

        // match parameters {
        //     Some(p) => req = req.json(&p),
        //     None => req = req,
        // }

        if parameters.is_some() {
            req = req.json(&parameters.unwrap().inner());
        }

        println!("{:?}", req);
        let res = req.send().unwrap();

        let result_text = res.text_with_charset("utf-8").unwrap();
        println!("{:?}", result_text);
        // Ok(serde_json::from_str::(&result_text.as_str()).unwrap())
        Ok(result_text)
    }
}
