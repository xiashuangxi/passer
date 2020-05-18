use reqwest::{self, blocking, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{Client, ClientError, InternalError, Parameter, RequestError};

#[derive(Debug)]
pub struct Request<'a>(&'a Client);

impl<'a> Request<'a> {
    pub fn new(client: &'a Client) -> Request<'a> {
        Request(client)
    }

    pub fn send<T>(
        &self,
        url: &str,
        method: reqwest::Method,
        parameters: Option<Box<dyn Parameter>>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let mut req = blocking::Client::new()
            .request(method, &format!("{}/{}", &self.0.home.to_string(), url));
        for x in self.0.header.inner().iter() {
            req = req.header(x.name(), x.value());
        }
        if parameters.is_some() {
            req = req.json(&parameters.unwrap().inner());
        }

        let res = match req.send() {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        let status = res.status();

        let result_text = res.text_with_charset("utf-8").unwrap();

        if status.is_client_error() {
            RequestError::new(result_text.as_str())?
        }
        if status.is_server_error() {
            InternalError::new(result_text.as_str())?
        }
        // Ok(serde_json::from_str::<T>(&result_text).unwrap())

        Ok(serde_json::from_str::<T>(result_text.as_str()).unwrap())

        // serde_json::from_str::<T>(result_text)
    }
}
