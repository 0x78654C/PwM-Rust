
#[cfg(test)]
#[path="./libs/json_lib.rs"]
mod json_lib;

#[test]
fn serialize_test(){
   let app = String::from("facebook");
   let acc = String::from("x_coding");
   let pass = String::from("superpassword1234");
   let serilized_data = "{\"application\":\"facebook\",\"account\":\"x_coding\",\"password\":\"superpassword1234\"}";
   let json =json_lib::json_serialize(app, acc, pass);
   assert_eq!(serilized_data,json);
}

#[test]
fn deserialize_test(){
    let data  = String::from("facebook|x_coding|superpassword1234");
    let serilized_data = "{\"application\":\"facebook\",\"account\":\"x_coding\",\"password\":\"superpassword1234\"}";
    let deserialized_data = json_lib::json_deserialize(serilized_data);
    assert_eq!(data,deserialized_data);
}