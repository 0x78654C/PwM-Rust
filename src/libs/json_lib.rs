use std::fmt::format;

use serde::{Deserialize, Serialize, Deserializer};
use serde_json::Result;

#[derive(Serialize,Deserialize)]
struct key_values {
    application: String,
    account:String,
    password:String
}

// Serialize app data.
pub fn json_serialize(app:String, acc:String, pass:String)->String{
    let app_data = key_values{
        application: app,
        account:acc,
        password:pass
    };

    let serialize = serde_json::to_string(&app_data).unwrap();
    return serialize;
}

// Deserialize app data.
pub fn json_deserialize(serialized_data:&str)->String{
    let deserialized_data:key_values =  serde_json::from_str(serialized_data).unwrap();
    let des_data = format!("{}|{}|{}",deserialized_data.application, deserialized_data.account, deserialized_data.password);
    return des_data;
}
