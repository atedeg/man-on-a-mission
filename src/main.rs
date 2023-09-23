// Clippy stuff
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::integer_division)]
#![warn(clippy::string_slice)]
#![warn(clippy::string_to_string)]
#![warn(clippy::try_err)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod commands;
mod operations;
use std::time::Duration;

use crate::operations::schema;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use teloxide::{prelude::Dispatcher, Bot};
use tracing::info;

#[tokio::main]
async fn main() {
    // Load .env file, if present
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    let db_connection_string = dotenvy::var("DATABASE_URL")
        .context("DATABASE_URL must be set")
        .unwrap();
    info!("connecting to database: {}", db_connection_string);
    let db_pool = PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&db_connection_string)
        .await
        .context("Could not connect to database")
        .unwrap();

    info!("running migrations");
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .context("Could not run migrations")
        .unwrap();

    let bot = Bot::from_env();
    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
