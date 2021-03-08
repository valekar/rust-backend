use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgConnectOptions;
use sqlx::Pool;
use sqlx::{postgres::Postgres, query, PgPool};
use std::{borrow::Borrow, env::var};
//use tide::Body;
use tide::Response;
use tide::{http::StatusCode, Body, Request, Server};

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    //let db_url = std::env::var("DATABASE_URL")?;
    //let pool: PgPool = Pool::<Postgres>::connect(&db_url).await?;

    let app = server().await;

    println!("Hello, world!");

    app.listen("127.0.0.1:8080").await.unwrap();
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DBError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}

async fn server() -> Server<State> {
    let host = var("HOST").unwrap();
    let port: u16 = var("PORT").unwrap().parse::<u16>().unwrap();
    let password = var("PASSWORD").unwrap();
    let username = var("USERNAME").unwrap();
    let database = var("DATABASE").unwrap();

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    let db_pool: PgPool = Pool::<Postgres>::connect_with(pool_options).await.unwrap();
    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move {
        let pool = &req.state().db_pool;
        let rows = query!("select count(*) from users").fetch_one(pool).await?;

        dbg!(rows);

        //let json = ([1, 2, 3]);

        //let res = Response::new(StatusCode::Ok);
        //res.set_body(Body::from_json(&json))
        //Ok(res.body_json(&json)?)
        //Ok("Hello world")

        let cat = Cat {
            name: String::from("Srinivas"),
        };

        let mut res: Response = Response::new(StatusCode::Ok);
        &res.set_body(Body::from_json(&cat)?);

        Ok(res)
    });

    app
}

#[derive(Debug, Deserialize, Serialize)]
struct Cat {
    name: String,
}

#[cfg(test)]
mod test {}
