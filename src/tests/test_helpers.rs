mod test_db;

use crate::State;
use crate::{server, Server};
use futures::{executor::block_on, prelude::*};
pub use serde_json::{json, Value};
use std::collections::HashMap;
use std::pin::Pin;
use test_db::TestDb;
use tide::{
    http::{Request, Response, Url},
    StatusCode,
};

use serde::de::DeserializeOwned;

pub async fn test_setup() -> TestServer {
    let test_db = TestDb::new().await;
    let db_pool = test_db.db();

    let server = server(db_pool).await;
    TestServer::new(server, test_db)
}

pub struct TestServer {
    server: Server<State>,
    test_db: TestDb,
}

impl TestServer {
    fn new(server: Server<State>, test_db: TestDb) -> Self {
        Self { server, test_db }
    }

    pub async fn simulate(&self, req: Request) -> tide::Result<Response> {
        self.server.respond(req).await
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
            println!("body =>>>> {}", body);
            Ok(serde_json::from_str(&body)?)
        })
    }
}

#[derive(Debug)]
pub struct TestRequest {
    url: String,
    headers: HashMap<String, String>,
    kind: TestRequestKind,
}

#[derive(Debug)]
pub enum TestRequestKind {
    Get,
}

impl TestRequest {
    pub async fn send(self, server: &TestServer) -> (Value, StatusCode, HashMap<String, String>) {
        let url = Url::parse(&format!("http://localhost:8080{}", self.url)).unwrap();

        let mut req = match self.kind {
            TestRequestKind::Get => Request::new(tide::http::Method::Get, url),
        };

        let req_copy = req.clone();
        for (key, value) in self.headers {
            req.append_header(key.as_str(), value.as_str());
        }

        let resp = server.simulate(req).await;

        let resp_copy = server.simulate(req_copy).await;

        let res = resp.unwrap();

        match resp_copy {
            Ok(_) => {
                // res = result;
            }
            Err(err) => {
                print!("Errrrr {}", err)
            }
        }

        //let res = resp.unwrap();

        let status = res.status();
        let headers = res
            .iter()
            .flat_map(|(key, values)| {
                values
                    .iter()
                    .map(move |value| (key.as_str().to_string(), value.as_str().to_string()))
            })
            .collect::<HashMap<_, _>>();

        let json = res.body_json::<Value>().await.unwrap();

        let json = json!(["1,23"]);

        (json, status, headers)
    }
}

pub fn get(url: &str) -> TestRequest {
    TestRequest {
        url: url.to_string(),
        headers: HashMap::new(),
        kind: TestRequestKind::Get,
    }
}
