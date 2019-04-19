#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate google_signin;

use std::env;
use std::sync::Mutex;
use std::thread;
use std::time;

use log::debug;
use log::error;
use log::info;
use log::warn;

use diesel::prelude::*;
use diesel::MysqlConnection;

use dotenv::dotenv;

mod access;
mod errors;
mod search;
mod users;

use web_dev::errors::WebdevError;
use web_dev::errors::WebdevErrorKind;

use web_dev::users::models::UserRequest;
use web_dev::users::requests::handle_user;

use access::models::{AccessRequest, UserAccessRequest};
use access::requests::get_user;
use access::requests::{handle_access, handle_user_access};

embed_migrations!();

fn main() {
    dotenv().ok();

    simplelog::SimpleLogger::init(simplelog::LevelFilter::Trace, simplelog::Config::default())
        .unwrap();

    info!("Connecting to database");

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_e) => {
            error!("Could not read DATABASE_URL environment variable");
            return;
        }
    };

    debug!("Connecting to {}", database_url);

    let connection = loop {
        match MysqlConnection::establish(&database_url) {
            Ok(c) => break c,
            Err(e) => {
                warn!("Could not connect to database: {}", e);
                info!("Retrying in a second");
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    };

    debug!("Connected to database");

    info!("Running migrations");
    embedded_migrations::run(&connection);

    let connection_mutex = Mutex::new(connection);

    info!("Starting server on 0.0.0.0:8000");

    rouille::start_server("0.0.0.0:8000", move |request| {
        debug!(
            "Handling request {} {} from {}",
            request.method(),
            request.raw_url(),
            request.remote_addr()
        );

        if request.method() == "OPTIONS" {
            rouille::Response::text("")
                .with_additional_header(
                    "Access-Control-Allow-Methods",
                    "POST, GET, DELETE, OPTIONS",
                )
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Headers", "X-PINGOTHER, Content-Type")
                .with_additional_header("Access-Control-Max-Age", "86400")
        } else {
            let current_connection = match connection_mutex.lock() {
                Ok(c) => c,
                Err(_e) => {
                    error!("Could not lock database");
                    return rouille::Response::from(WebdevError::new(WebdevErrorKind::Database));
                }
            };

            let response = handle_request(request, &current_connection);

            response.with_additional_header("Access-Control-Allow-Origin", "*")
        }
    });
}

fn handle_request(
    request: &rouille::Request,
    database_connection: &MysqlConnection,
) -> rouille::Response {
    let mut requesting_user = None;

    if let Some(id_token) = request.header("id_token") {
        requesting_user = get_user(id_token, database_connection);
    }

    if let Some(user_request) = request.remove_prefix("/users") {
        match UserRequest::from_rouille(&user_request) {
            Err(err) => rouille::Response::from(err),
            Ok(user_request) => {
                match handle_user(user_request, requesting_user, database_connection) {
                    Ok(user_response) => user_response.to_rouille(),
                    Err(err) => rouille::Response::from(err),
                }
            }
        }
    } else if let Some(access_request) = request.remove_prefix("/access") {
        match AccessRequest::from_rouille(&access_request) {
            Err(err) => rouille::Response::from(err),
            Ok(access_request) => {
                match handle_access(access_request, requesting_user, database_connection) {
                    Ok(access_response) => access_response.to_rouille(),
                    Err(err) => rouille::Response::from(err),
                }
            }
        }
    } else if let Some(user_access_request) = request.remove_prefix("/user_access") {
        match UserAccessRequest::from_rouille(&user_access_request) {
            Err(err) => rouille::Response::from(err),
            Ok(user_access_request) => {
                match handle_user_access(user_access_request, requesting_user, database_connection)
                {
                    Ok(user_access_response) => user_access_response.to_rouille(),
                    Err(err) => rouille::Response::from(err),
                }
            }
        }
    } else {
        rouille::Response::empty_404()
    }
}
