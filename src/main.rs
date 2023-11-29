const DB_FILE: &str = "shorts.db";

use std::{sync::Arc, process::exit};

use database::database::setup_table;
use http_serv::{self, http::{server::HttpServer, http_structs::{HttpResponse, HttpData, HttpStatus, HttpRequest}}};
use r2d2_sqlite::SqliteConnectionManager;

use crate::{database::database::{get_details, delete_short_uri, add_real_uri, add_detail, fetch_real_uri}, structs::structs::{DetailEntry, Shorten}};

mod structs;
mod database;

extern crate pretty_env_logger;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_timed();

    let sqlite_connection_manager = SqliteConnectionManager::file(DB_FILE);
    let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager).expect("Failed to create connection pool");
    let pool_arc = Arc::new(sqlite_pool);

    match setup_table(pool_arc.get().expect("Failed to capture connection pointer")) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Failed to setup tables: {err}");
            exit(-1);
        },
    }


    let mut server = HttpServer::new("0.0.0.0".to_string(), "8443".to_string(), None, Vec::new(), None)?;
    
    // follow the requested uri
    let clone = pool_arc.clone();
    server.get("/:s_uri".to_owned(), Box::new(move |request: HttpRequest| {
        log::trace!("Useragent was {}", request.get_extra_header(String::from("User-Agent")).unwrap_or(String::from("not found")));
        let connection = match clone.clone().get() {
            Ok(connection) => connection,
            Err(err) => {
                log::error!("Failed to get connection pointer: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::PreconditionFailed,
                    None,
                    Some(HttpData::new(format!("Failed to get connection pointer: {err}").into_bytes()))
                )
            }
        };

        let s_uri = match request.get_route_param(String::from(":s_uri")) {
            Some(s_uri) => s_uri,
            None => {
                log::error!("No short URI provided");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::ExpectationFailed,
                    None,
                    Some(HttpData::new(String::from("No short URI provided").into_bytes()))
                )
            }
        };

        let r_uri = match fetch_real_uri(connection, s_uri.clone()) {
            Ok(r_uri) => {
                r_uri
            },
            Err(err) => {
                log::error!("Failed to fetch URI mapping: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::NotFound,
                    None,
                    Some(HttpData::new(format!("Failed to fetch URI mapping: {err}").into_bytes()))
                )
            }
        };

        let connection = match clone.clone().get() {
            Ok(connection) => connection,
            Err(err) => {
                log::error!("Failed to get connection pointer: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::PreconditionFailed,
                    None,
                    Some(HttpData::new(format!("Failed to get connection pointer: {err}").into_bytes()))
                )
            }
        };

        match add_detail(connection, s_uri, request.client_ip) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Failed to delete URI mapping: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::InternalServerError,
                    None,
                    Some(HttpData::new(format!("Failed to delete URI mapping: {err}").into_bytes()))
                )
            }
        }

        log::trace!("Redirecting to {r_uri}");
        HttpResponse::new(
            String::from("1.1"),
            HttpStatus::TemporaryRedirect,
            Some(vec![("Location".to_owned(), format!("{r_uri}"))]),
            None
        )
    }));

    // shorten given uri
    let clone = pool_arc.clone();
    server.post("/_shorten".to_owned(), Box::new(move |request: HttpRequest| {
        log::trace!("Useragent was {}", request.get_extra_header(String::from("User-Agent")).unwrap_or(String::from("not found")));
        let connection = match clone.clone().get() {
            Ok(connection) => connection,
            Err(err) => {
                log::error!("Failed to get connection pointer: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::PreconditionFailed,
                    None,
                    Some(HttpData::new(format!("Failed to get connection pointer: {err}").into_bytes()))
                )
            }
        };

        let r_uri: Shorten = match request.data {
            Some(http_data) => {
                match String::from_utf8(http_data.data) {
                    Ok(r_uri) => {
                        match serde_json::from_str(&r_uri) {
                            Ok(r_uri) => r_uri,
                            Err(err) => {
                                log::error!("Failed to deserialize data: {err}");
                                return HttpResponse::new(
                                    String::from("1.1"),
                                    HttpStatus::PreconditionFailed,
                                    None,
                                    Some(HttpData::new(format!("Failed to deserialize data: {err}").into_bytes()))
                                )
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("Failed to deserialize data: {err}");
                        return HttpResponse::new(
                            String::from("1.1"),
                            HttpStatus::PreconditionFailed,
                            None,
                            Some(HttpData::new(format!("Failed to deserialize data: {err}").into_bytes()))
                        )
                    }
                }
            },
            None => {
                log::error!("No real URI provided");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::ExpectationFailed,
                    None,
                    Some(HttpData::new(String::from("No real URI provided").into_bytes()))
                )
            }
        };

        match add_real_uri(connection, r_uri.r_uri) {
            Ok(s_uri) => {
                HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::Ok,
                    None,
                    Some(HttpData::new(s_uri.into_bytes()))
                )
            },
            Err(err) => {
                log::error!("Failed to delete URI mapping: {err}");
                HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::InternalServerError,
                    None,
                    Some(HttpData::new(format!("Failed to delete URI mapping: {err}").into_bytes()))
                )
            }
        }
    }));

    // delete a shortlink
    let clone = pool_arc.clone();
    server.delete("/_delete/:s_uri".to_owned(), Box::new(move |request: HttpRequest| {
        log::trace!("Useragent was {}", request.get_extra_header(String::from("User-Agent")).unwrap_or(String::from("not found")));
        let connection = match clone.clone().get() {
            Ok(connection) => connection,
            Err(err) => {
                log::error!("Failed to get connection pointer: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::PreconditionFailed,
                    None,
                    Some(HttpData::new(format!("Failed to get connection pointer: {err}").into_bytes()))
                )
            }
        };

        let s_uri = match request.get_route_param(String::from(":s_uri")) {
            Some(s_uri) => s_uri,
            None => {
                log::error!("No short URI provided");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::ExpectationFailed,
                    None,
                    Some(HttpData::new(String::from("No short URI provided").into_bytes()))
                )
            }
        };

        match delete_short_uri(connection, s_uri) {
            Ok(_) => HttpResponse::new(
                String::from("1.1"),
                HttpStatus::Ok,
                None,
                None
            ),
            Err(err) => {
                log::error!("Failed to delete URI mapping: {err}");
                HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::InternalServerError,
                    None,
                    Some(HttpData::new(format!("Failed to delete URI mapping: {err}").into_bytes()))
                )
            }
        }
    }));

    // fetches info about the shortened uri in 100 entry pages
    let clone = pool_arc.clone();
    server.get("/_info/:s_uri/:page".to_owned(), Box::new(move|request: HttpRequest| {
        log::trace!("Useragent was {}", request.get_extra_header(String::from("User-Agent")).unwrap_or(String::from("not found")));
        let connection = match clone.clone().get() {
            Ok(connection) => connection,
            Err(err) => {
                log::error!("Failed to get connection pointer: {err}");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::PreconditionFailed,
                    None,
                    Some(HttpData::new(format!("Failed to get connection pointer: {err}").into_bytes()))
                )
            }
        };

        let s_uri = match request.get_route_param(String::from(":s_uri")) {
            Some(s_uri) => s_uri,
            None => {
                log::error!("No short URI provided");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::BadRequest,
                    None,
                    Some(HttpData::new(String::from("No short URI provided").into_bytes()))
                )
            }
        };

        let page = match request.get_route_param(String::from(":page")) {
            Some(s_uri) => match s_uri.parse::<usize>() {
                Ok(page) => {
                    if page <= 0 {
                        return HttpResponse::new(
                        String::from("1.1"),
                        HttpStatus::BadRequest,
                        None,
                        Some(HttpData::new(String::from("Invalid page identifier").into_bytes()))
                        )
                    }
                    page
                },
                Err(_) => {
                    log::error!("Invalid page identifier");
                    return HttpResponse::new(
                        String::from("1.1"),
                        HttpStatus::BadRequest,
                        None,
                        Some(HttpData::new(String::from("Invalid page identifier").into_bytes()))
                    )
                },
            },
            None => {
                log::error!("No page provided");
                return HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::BadRequest,
                    None,
                    Some(HttpData::new(String::from("No page provided").into_bytes()))
                )
            }
        };

        match get_details(connection, s_uri, page, 100 as usize) {
            Ok(details) => {
                let details_structs = details.iter().map(|elem| {
                    DetailEntry::new(elem.1.clone(), format!("{}", elem.0), elem.2.clone())
                }).collect::<Vec<DetailEntry>>();
                let serialized_structs: String = match serde_json::to_string(&details_structs) {
                    Ok(serialized_structs) => serialized_structs,
                    Err(err) => {
                        log::error!("Failed to serialize data: {err}");
                        return HttpResponse::new(
                            String::from("1.1"),
                            HttpStatus::InternalServerError,
                            None,
                            Some(HttpData::new(format!("Failed to parse data: {err}").into_bytes()))
                        )
                    }
                };
                HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::Ok,
                    None,
                    Some(HttpData::new(serialized_structs.into_bytes()))
                )
            },
            Err(err) => {
                log::error!("Failed to fetch details : {err}");
                HttpResponse::new(
                    String::from("1.1"),
                    HttpStatus::InternalServerError,
                    None,
                    Some(HttpData::new(format!("Failed to fetch details: {err}").into_bytes()))
                )
            }
        }
    }));

    server.run_loop()?;
    Ok(())
}

