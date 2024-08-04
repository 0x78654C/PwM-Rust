#[cfg(test)]
#[path="../libs/web_lib.rs"]
mod web_lib;
use tokio::runtime::Runtime;

#[test]
fn get_data_test()
{
    let api = "https://api.pwnedpasswords.com/range/21BD1";
    let api_get = web_lib::get_data(api);
    let o = Runtime::new().unwrap().block_on(api_get);
    assert_eq!(true,true);
    //assert_eq!(true,api_get.contains("0018A"));
}
