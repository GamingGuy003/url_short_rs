// static DBURL: &str = "db";
// static TABLE: &str = "translations";

use http_serv::{self, http::{server::HttpServer, http_structs::{HttpResponse, HttpData}}};

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("0.0.0.0".to_string(), "8443".to_string(), Vec::new())?;
    server.get("/:test1/_ding".to_owned(), |request| {
        println!("singletest");
        HttpResponse::default()
    });
    server.get("/:test2/_ding/:test1".to_owned(), |request| {
        let mut resp = HttpResponse::default();
        resp.data = Some(HttpData::new(format!("{:#?}", request).as_bytes().to_vec()));
        resp
    });
    server.run_loop()?;
    Ok(())
}
