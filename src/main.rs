use http_client::call_a_http_web_page;
use https_client::call_a_https_web_page;
use http_from_https_client::call_a_http_web_page_from_https_client;

pub mod http_client;
pub mod https_client;
pub mod http_from_https_client;

/// This code is for fun
fn main() {
    println!("Hello, world!");

    // Comment following lines if there's no localhost http server running
    let response = call_a_http_web_page();
    println!("Printing from test case {}", response);

    // Comment following if there's no localhost https server running
    let response = call_a_https_web_page();
    println!("Priting from test case {}", response);

    // Comment following lines if there's no localhost http server running
    let response = call_a_http_web_page_from_https_client();
    println!("Printing from test case {}", response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_a_web_page_test() {
        let response = call_a_http_web_page();
        debug_assert!(!response.is_empty())
    }

    #[test]
    fn call_a_secured_web_page_test() {
        let response = call_a_https_web_page();
        debug_assert!(!response.is_empty())
    }

    #[test]
    fn call_a_insecured_web_page_from_secured_client_test() {
        let response = call_a_http_web_page_from_https_client();
        debug_assert!(!response.is_empty())
    }
}