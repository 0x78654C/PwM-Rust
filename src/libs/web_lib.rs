extern crate hyper;
use hyper::Request;

// Get body data from web GET request
pub  fn get_data(url: &str) -> String{
   
    let req = Request::builder()
    .uri(url)
    .header(hyper::header::HOST, authority.as_str())
    .body(Empty::<Bytes>::new())
    return body;
}
