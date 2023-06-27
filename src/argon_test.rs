#[cfg(test)]
//extern crate argon2;
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
    let salt = pass_substring.as_bytes();
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 40,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    println!{"{}", pass_substring};
    println!("{:?}", salt);
    //let hash_enc = encode(hash);
    //let matches = argon2::verify_encoded(&hash, password).unwrap();
    assert_eq!(hash, hash_result);
}
