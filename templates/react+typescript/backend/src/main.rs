mod controllers;
mod schemas;
mod utils;

use std::process::exit;

use actix_files as fs;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpRequest, HttpServer,
};
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

use controllers::{healthcheck::check, webhooks::clerk_webhook};
struct AppState {
    client: Clerk,
    db_pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let clerk_secret_key =
        std::env::var("CLERK_SECRET_KEY").expect("Clerk secret key not provided");

    let db_url = std::env::var("DB_URL").expect("Database url not provided");

    let db_pool = match sqlx::postgres::PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Successfully connected to Db");
            pool
        }
        Err(err) => {
            println!("{:#?}", err);
            exit(1)
        }
    };

    let clerk_config = ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
    let client = Clerk::new(clerk_config.clone());

    // runs migrations
    match sqlx::migrate!("./migrations").run(&db_pool).await {
        Ok(_) => println!("Migrations made successfully"),
        Err(err) => {
            println!("Error while making migrations \n {:#?}", err);
            std::process::exit(1)
        }
    }

    let app_state = web::Data::new(AppState {
        client,
        db_pool: db_pool.clone(),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(app_state.clone())
            .wrap(logger)
            .service(web::scope("/api").service(check))
            .service(clerk_webhook)
            .service(fs::Files::new("/", "./build").index_file("index.html"))
            .default_service(web::route().to(fallback))
    })
    .bind(("127.0.0.1", 3000))
    .expect("Unable to start server")
    .run()
    .await
}

async fn fallback(req: HttpRequest) -> Result<fs::NamedFile, actix_web::Error> {
    Ok(fs::NamedFile::open("./build/index.html")?)
}
