use http::{Request, Response};


pub fn get_data(api: &str) -> String(){
    let mut request = Request::builder().uri(api);
    return request.body();
}