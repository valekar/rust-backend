use sqlx::postgres::PgConnectOptions;
use sqlx::Pool;
use sqlx::{postgres::Postgres, query, PgPool};
use std::env::var;
use tide::{Request, Server};

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    //let db_url = std::env::var("DATABASE_URL")?;
    //let pool: PgPool = Pool::<Postgres>::connect(&db_url).await?;

    let host = var("HOST")?;
    let port: u16 = var("PORT")?.parse::<u16>().unwrap();
    let password = var("PASSWORD")?;
    let username = var("USERNAME")?;
    let database = var("DATABASE")?;

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    let db_pool: PgPool = Pool::<Postgres>::connect_with(pool_options).await?;

    println!("Hello, world!");

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move {
        let pool = &req.state().db_pool;
        let rows = query!("select 1 as one").fetch_one(pool).await?;

        dbg!(rows);

        Ok("Hello world")
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
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
