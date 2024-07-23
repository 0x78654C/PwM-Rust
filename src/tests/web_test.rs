#[cfg(test)]
#[path="../libs/web_lib.rs"]
mod web_lib;

#[test]
fn get_data_test()
{
    let api = "https://api.pwnedpasswords.com/range/21BD1";
    let api_get = web_lib::get_data(api);
    assert_eq!(api_get.contains("0018A"));
}
