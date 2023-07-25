#[cfg(test)]
use argon2::{self, Config, ThreadMode, Variant, Version};
 

#[path="./libs/argon_lib.rs"]
mod argon_lib;

#[test]
fn password_match_simple_test(){
    let password = b"password";
    let salt = b"radonmsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password,salt , &config).unwrap();
    let matches = argon2::verify_encoded(&hash,password).unwrap(); 
    assert!(matches);
}

#[test]
fn password_match_pwm() {
    let password = "@#$ASDSA#$#@34234asdASDas";
    let hash_result = "p0XtEZ3yARyE0CfS+9nzZW17udJOTxJQRDdWSIMVnVA=";
    let argon_hash = argon_lib::argon_password_hash(password);
    //let enc = encode(argon_hash);
    println!("Encrypted: {}", argon_hash);
    assert_eq!(argon_hash, hash_result);
}
