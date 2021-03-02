use sqlx::postgres::PgConnectOptions;
use sqlx::Pool;
use sqlx::{postgres::Postgres, query, PgPool};
use std::env::var;
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

    let pool: PgPool = Pool::<Postgres>::connect_with(pool_options).await?;

    //let options =

    let rows = query!("select 1 as one").fetch_one(&pool).await?;

    dbg!(rows);
    //dbg!(db_url);

    println!("Hello, world!");

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello world") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
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
