const DB_FILE: &str = "shorts.db";

use std::{sync::Arc, process::exit, clone};

use http_serv::{self, http::{server::HttpServer, http_structs::{HttpResponse, HttpData, HttpStatus}}};
use r2d2::PooledConnection;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

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


    let mut server = HttpServer::new("0.0.0.0".to_string(), "8443".to_string(), None, Vec::new())?;

    let delete = pool_arc.clone();
    let info = pool_arc.clone();
    
    // follow the requested uri
    let clone = pool_arc.clone();
    server.get("/:uri".to_owned(), Box::new(|request| {
        clone;
        let mut resp = HttpResponse::new("1.1".to_string(), HttpStatus::MovedPermanently, Some(vec![("Location".to_owned(), "https://google.de".to_owned())]), None);
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    }));
    
    // delete a shortlink
    let clone = pool_arc.clone();
    server.post("/_shorten/:uri".to_owned(), Box::new(|request| {
        clone.get();
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    }));

    // delete a shortlink
    server.delete("/_delete/:uri".to_owned(), Box::new(|request| {
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    }));

    // fetches info about the shortened uri
    server.get("/_info/:uri".to_owned(), Box::new(|request| {
        log::warn!("Useragent was {}", request.get_extra_header(String::from("User-Agent")).unwrap_or(String::from("not found")));
        let resp = HttpResponse::default();
        resp
    }));
    server.run_loop()?;
    Ok(())
}

fn setup_table(connection: PooledConnection<SqliteConnectionManager>) -> Result<(), r2d2_sqlite::rusqlite::Error> {
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS uri_map (
                suri INTEGER NOT NULL PRIMARY KEY,
                ruri TEXT(2048) NOT NULL
            )", [])
    {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Created table uri_map")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while setting up table uri_map: {err}");
            return Err(err);
        }
    }

    match connection.execute(
        "CREATE TABLE IF NOT EXISTS uri_details (
                suri INT NOT NULL,
                clientip TEXT(128) NOT NULL,
                time TIMESTAMP NOT NULL,
                FOREIGN KEY(suri) REFERENCES uri_map(suri)
            )", [])
    {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Created table uri_details")
            }
            Ok(())
        },
        Err(err) => {
            log::trace!("Encountered error while setting up table uri_details: {err}");
            Err(err)
        }
    }
}

fn fetch_real_uri(connection: PooledConnection<SqliteConnectionManager>, suri: String) -> Result<String, r2d2_sqlite::rusqlite::Error> {
    let mut stmt = match connection.prepare("SELECT ruri FROM  uri_map WHERE suri = ?1") {
        Ok(stmt) => stmt,
        Err(err) => {
            log::trace!("Failed to prepare statement for fetching rURI for {suri}: {err}");
            return Err(err);
        }
    };

    match stmt.query_row(params![suri], |row| row.get(0)) {
        Ok(ruri) => Ok(ruri),
        Err(err) => {
            log::trace!("Failed to get rURI: {err}");
            Err(err)
        }
    }
}

fn add_real_uri(connection: PooledConnection<SqliteConnectionManager>, ruri: String) -> Result<String, r2d2_sqlite::rusqlite::Error> {
    match connection.execute("INSERT INTO uri_map (ruri) VALUES (?1)", params![ruri]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Inserted {ruri} into uri_map")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while inserting rURI into uri_map: {err}");
            return Err(err)
        }
    }
    let suri: Result<i32, r2d2_sqlite::rusqlite::Error> = connection.query_row("SELECT last_insert_rowid()", [], |row| row.get(0));
    match suri {
        Ok(suri) => Ok(format!("{suri}")),
        Err(err) => {
            log::trace!("Failed to fetch sURI: {err}");
            Err(err)
        }
    }
}

fn delete_short_uri(connection: PooledConnection<SqliteConnectionManager>, suri: String) -> Result<(), r2d2_sqlite::rusqlite::Error> {
    match connection.execute("DELETE FROM uri_map WHERE suri = ?1", params![suri]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Removed {suri} from uri_map")
            } else {
                log::debug!("No entry to remove with sURI {suri}")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while removing sURI from uri_map: {err}");
            return Err(err)
        }
    }
    Ok(())
}