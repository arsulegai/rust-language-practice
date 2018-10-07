extern crate futures;
extern crate hyper;
extern crate tokio_core;

use self::futures::future;
use self::futures::stream::Stream;
use self::hyper::Client;
use self::hyper::client::ResponseFuture;
use self::hyper::rt::Future;
use self::hyper::Uri;
use self::tokio_core::reactor::Core;

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

pub fn call_a_http_web_page() -> String {
    let client = Client::new();
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
        _ => panic!("Couldn't find"),
    }
}