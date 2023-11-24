// static DBURL: &str = "db";
// static TABLE: &str = "translations";

use http_serv::{self, http::{server::HttpServer, http_structs::{HttpResponse, HttpData, HttpStatus}}};

extern crate pretty_env_logger;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_timed();
    let mut server = HttpServer::new("0.0.0.0".to_string(), "8443".to_string(), Vec::new())?;
    server.get("/:uri".to_owned(), |_| {
        let resp = HttpResponse::new("1.1".to_string(), HttpStatus::MovedPermanently, Some(vec![("Location".to_owned(), "https://google.de".to_owned())]), None);
        resp
    });
    server.post("/_shorten/:uri".to_owned(), |request| {
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    });
    server.delete("/_delete/:uri".to_owned(), |request| {
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    });
    server.get("/_info/:uri".to_owned(), |request| {
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    });
    server.run_loop()?;
    Ok(())
}
