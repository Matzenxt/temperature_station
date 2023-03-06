extern crate core;

use std::{io, env};
use actix_cors::Cors;
use actix_web::{App, http, HttpServer, middleware};
use actix_web::web::Data;
use sqlx::sqlite::SqlitePoolOptions;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::service::measurement::{add_measurement, get_all_measurements, get_by_search};
use crate::service::room::get_all_room;

mod impls;
mod service;
mod traits;
mod helper;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set in .env file");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT is not set in .env file");
    let database = env::var("DATABASE_URL").expect("DATABASE is not set in .env file");
    let dir_private_key = env::var("DIR_PRIVATE_KEY").expect("DIR_PRIVATE_KEY is not set in .env file");
    let dir_certificate_chain = env::var("DIR_CERTIFICATE_CHAIN").expect("DIR_CERTIFICATE_CHAIN is not set in .env file");

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    /*
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(dir_private_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(dir_certificate_chain).unwrap();
*/

    let pool = SqlitePoolOptions::new()
        .max_connections(60)
        .connect(database.as_str()).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))

        // Services
        .service(add_measurement)
        .service(get_all_measurements)
        .service(get_by_search)
        .service(get_all_room)
    })
        .bind(format!("{}:{}", server_address, server_port))?
        //.bind_openssl(format!("{}:{}", server_address, server_port), builder)?
        .run()
        .await
}
