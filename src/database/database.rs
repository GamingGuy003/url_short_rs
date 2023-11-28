use std::net::SocketAddr;

use r2d2::PooledConnection;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};

pub fn setup_table(connection: PooledConnection<SqliteConnectionManager>) -> Result<(), r2d2_sqlite::rusqlite::Error> {
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS uri_map (
                s_uri INTEGER NOT NULL PRIMARY KEY,
                r_uri TEXT(2048) NOT NULL
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
                s_uri INT NOT NULL,
                client_ip TEXT(128) NOT NULL,
                time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(s_uri) REFERENCES uri_map(s_uri)
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

pub fn fetch_real_uri(connection: PooledConnection<SqliteConnectionManager>, s_uri: String) -> Result<String, r2d2_sqlite::rusqlite::Error> {
    let mut stmt = match connection.prepare("SELECT r_uri FROM  uri_map WHERE s_uri = ?1") {
        Ok(stmt) => stmt,
        Err(err) => {
            log::trace!("Failed to prepare statement for fetching r_uri for {s_uri}: {err}");
            return Err(err);
        }
    };

    match stmt.query_row(params![s_uri], |row| row.get(0)) {
        Ok(r_uri) => Ok(r_uri),
        Err(err) => {
            log::trace!("Failed to get r_uri: {err}");
            Err(err)
        }
    }
}

pub fn add_real_uri(connection: PooledConnection<SqliteConnectionManager>, r_uri: String) -> Result<String, r2d2_sqlite::rusqlite::Error> {
    match connection.execute("INSERT INTO uri_map (r_uri) VALUES (?1)", params![r_uri]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Inserted {r_uri} into uri_map")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while inserting r_uri into uri_map: {err}");
            return Err(err)
        }
    }
    let s_uri: Result<i32, r2d2_sqlite::rusqlite::Error> = connection.query_row("SELECT last_insert_rowid()", [], |row| row.get(0));
    match s_uri {
        Ok(s_uri) => Ok(format!("{s_uri}")),
        Err(err) => {
            log::trace!("Failed to fetch s_uri: {err}");
            Err(err)
        }
    }
}

pub fn delete_short_uri(connection: PooledConnection<SqliteConnectionManager>, s_uri: String) -> Result<(), r2d2_sqlite::rusqlite::Error> {
    match connection.execute("DELETE FROM uri_map WHERE s_uri = ?1", params![s_uri]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Removed {s_uri} from uri_map")
            } else {
                log::debug!("No entry to remove with s_uri {s_uri}")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while removing s_uri from uri_map: {err}");
            return Err(err)
        }
    }
    match connection.execute("DELETE FROM uri_details WHERE s_uri = ?1", params![s_uri]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Removed {s_uri} from uri_details")
            } else {
                log::debug!("No entry to remove with s_uri {s_uri}")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while removing s_uri from uri_details: {err}");
            return Err(err)
        }
    }
    Ok(())
}

pub fn add_detail(connection: PooledConnection<SqliteConnectionManager>, s_uri: String, client_ip: SocketAddr) -> Result<(), r2d2_sqlite::rusqlite::Error> {
    match connection.execute("INSERT INTO uri_details (s_uri, client_ip) VALUES (?1, ?2)", params![s_uri, client_ip.to_string()]) {
        Ok(cols) => {
            if cols != 0 {
                log::debug!("Inserted {client_ip}: {s_uri} into uri_details")
            }
        },
        Err(err) => {
            log::trace!("Encountered error while inserting {client_ip}: {s_uri} into uri_details: {err}");
            return Err(err)
        }
    }
    Ok(())
}

pub fn get_details(connection: PooledConnection<SqliteConnectionManager>, s_uri: String, page: usize, page_size: usize) -> Result<Vec<(i32, String, String)>, r2d2_sqlite::rusqlite::Error> {
    let mut details = Vec::new();
    let mut stmt = match connection.prepare("SELECT s_uri, client_ip, time FROM uri_details WHERE s_uri = ?1 ORDER BY time LIMIT ?2 OFFSET ?3") {
        Ok(stmt) => stmt,
        Err(err) => {
            log::trace!("Failed to prepare statement for fetching r_uri for {s_uri}: {err}");
            return Err(err);
        }
    };

    let rows = stmt.query(params![s_uri, page_size, ((page - 1) * page_size)]);
    match rows {
        Ok(mut rows) => {
            while let Some(row) = rows.next()? {
                details.push((row.get(0)?, row.get(1)?, row.get(2)?));
            }
            Ok(details)
        },
        Err(err) => {
            log::trace!("Failed to fetch rows: {err}");
            Err(err)
        }
    }
}