#[cfg(test)]
extern crate lib;
use argon2::{self, Config, ThreadMode, Variant, Version};
use base64::encode;


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
    let password = "password1234";
    let pass_substring = &password[2..12];
    let hash_result = "p0XtEZ3yARyE0CfS+9nzZW17udJOTxJQRDdWSIMVnVA=";

    assert_eq!(hash, hash_result);
}
