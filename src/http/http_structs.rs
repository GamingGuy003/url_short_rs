pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub protocol: String
}

pub enum Method {
    POST,
    PUT,
    GET,
    DELETE
}