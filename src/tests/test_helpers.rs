use crate::Server;
use crate::State;
use futures::{executor::block_on, prelude::*};
use std::pin::Pin;
use tide::http::{Response, Request};

use serde::de::DeserializeOwned;

pub struct TestServer {
    service: Server<State>,
    //test_db : TestDb
}

impl TestServer {
    fn new(service: Server<State> ) -> Self {
        Self { service}
    }

    pub async fn simulate(&self, req: Request) -> tide::Result<Response> {
        self.service.respond(req).await
    }
}

pub trait BodyJson {
    fn body_json<T: DeserializeOwned>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>>;
}

impl BodyJson for Response {
    fn body_json<T: DeserializeOwned>(
        mut self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>> {
        Box::pin(async move {
            let body = self.body_string().await?;
            println!("body = {}", body);
            Ok(serde_json::from_str(&body)?)
        })
    }
}
