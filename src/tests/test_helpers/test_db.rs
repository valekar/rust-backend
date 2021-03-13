//use sqlx::prelude::Connect;
use sqlx::{postgres::Postgres, query, PgPool, Pool};
use sqlx::{
    postgres::{PgConnectOptions, PgQueryResult},
    Connection, PgConnection,
};
use std::{env::var, fmt::Formatter};

#[derive(Debug)]
pub struct TestDb {
    pg_options: PgConnectOptions,
    db_pool: Option<PgPool>,
    db_name: String,
    schema_name: String,
}

impl TestDb {
    pub async fn new() -> Self {
        let db_url = &var("DATABASE_URL_TEST").unwrap();
        let schema_name = var("TEST_SCHEMA").unwrap();
        let db_name: String = var("TEST_DATABASE").unwrap().to_string();

        create_schema_and_set_schema(db_url, &schema_name).await;

        let pg_options = get_pg_options();
        let pg_options_clone = pg_options.clone();

        run_migrations(&pg_options).await;

        let db_pool: PgPool = Pool::<Postgres>::connect_with(pg_options).await.unwrap();

        Self {
            pg_options: pg_options_clone,
            db_pool: Some(db_pool),
            db_name: db_name,
            schema_name: schema_name,
        }
    }

    pub fn db(&self) -> PgPool {
        self.db_pool.clone().unwrap()
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = self.db_pool.take();
        futures::executor::block_on(drop_db(&self.pg_options, &self.schema_name, &self.db_name))
    }
}

// async fn create_db(db_url: &str, db_name: &str) {
//     println!("{}", db_url);
//     let mut conn = PgConnection::connect(db_url).await.unwrap();

//     let sql = format!(r#"CREATE DATABASE "{}""#, db_name);
//     sqlx::query::<Postgres>(&sql)
//         .execute(&mut conn)
//         .await
//         .unwrap();
// }

async fn create_schema_and_set_schema(db_url: &str, schema_name: &str) {
    println!("{}", db_url);
    let mut conn = PgConnection::connect(db_url).await.unwrap();

    let sql = format!(r#"CREATE SCHEMA {} "#, schema_name);

    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();

    let sql = format!(r#"SET SEARCH_PATH = {}"#, schema_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

// impl std::fmt::Display for PgQueryResult {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         Ok(self.rows_affected());
//         Err(println!(""))
//     }
// }

async fn run_migrations(pg_options: &PgConnectOptions) {
    let mut conn = PgConnection::connect_with(pg_options).await.unwrap();
    let sql = async_std::fs::read_to_string("./bin/backend/setup.sql")
        .await
        .unwrap();
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

fn get_pg_options() -> PgConnectOptions {
    let host = var("TEST_HOST").unwrap();
    let port: u16 = var("TEST_PORT").unwrap().parse::<u16>().unwrap();
    let password = var("TEST_PASSWORD").unwrap();
    let username = var("TEST_USER").unwrap();
    let database = var("TEST_DATABASE").unwrap();

    let pool_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .database(&database)
        .password(&password);

    pool_options
}

// fn generate_db_name(db_url: &str) -> String {
//     use rand::distributions::Alphanumeric;
//     use rand::{thread_rng, Rng};

//     let rng = thread_rng();
//     let suffix: String = rng.sample_iter(&Alphanumeric).take(16).collect();
//     format!("{}_{}", db_url, suffix)
// }

async fn drop_db(pg_options: &PgConnectOptions, schema_name: &str, db_name: &str) {
    println!("Dropping the schema with name {}", schema_name);

    let mut conn = PgConnection::connect_with(pg_options).await.unwrap();

    let sql = format!(
        r#"Select pg_terminate_backend(pg_stat_activity.pid) 
    FROM  pg_stat_activity 
    WHERE pg_stat_activity.datname = '{db_name}' 
    AND pid <> pg_backend_pid();"#,
        db_name = db_name
    );

    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();

    let sql = format!(r#"DROP SCHEMA "{schema}" CASCADE; "#, schema = schema_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}
