extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;

use self::futures::future;
use self::futures::stream::Stream;
use self::hyper::Client;
use self::hyper::client::ResponseFuture;
use self::hyper::rt::Future;
use self::hyper::Uri;
use self::tokio_core::reactor::Core;
use self::hyper::client::HttpConnector;
use self::native_tls::TlsConnector;
use self::hyper::Body;
use self::hyper_tls::HttpsConnector;
use self::native_tls::Certificate;
use std::fs::File;
use std::io::Read;

fn read_response(r: ResponseFuture) -> impl Future<Item=String, Error=hyper::Error> {
    let s = r
        .and_then(|res| {
            let inner_response = res.into_body()
                .fold(Vec::new(), |mut vector, chunk| {
                    vector.extend_from_slice(&chunk[..]);
                    future::ok::<_, hyper::Error>(vector)
                })
                .and_then(move |vectors| {
                    let body = String::from_utf8(vectors).unwrap();
                    future::ok(body)
                });
            inner_response
        });
    s
}

pub fn call_a_http_web_page_from_https_client() -> String {
    let mut file_contents = String::new();
    match File::open("/home/arun/WorkBench/webservers/server.pem") {
        Ok(mut file_present) => file_present.read_to_string(&mut file_contents),
        Err(error) => panic!("Unable to open the file {}", error),
    };
    let cert = match Certificate::from_pem(file_contents.as_bytes()) {
        Ok(cert_contents) => cert_contents,
        _ => panic!("Cert file is not valid"),
    };
    let tls_connector = match TlsConnector::builder().add_root_certificate(cert).build() {
        Ok(tls_connector_built) => tls_connector_built,
        _ => panic!("Unable to build TLS connector")
    };
    let mut http = HttpConnector::new(4);
    http.enforce_http(false);
    let https = HttpsConnector::from((http, tls_connector));
    let client = Client::builder().build::<_, Body>(https);
    let url = "http://localhost:8000".parse::<Uri>();
    let url_parsed = match url {
        Ok(url_parsed) => url_parsed,
        _ => panic!("Something terrible"),
    };
    let response = client.get(url_parsed);
    let mut runner = Core::new().unwrap();
    let result = runner.run(read_response(response));
    match result {
        Ok(found) => found,
        Err(not_found) => panic!("Couldn't find {}", not_found),
    }
}