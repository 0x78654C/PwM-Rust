#[cfg(test)]
#[path="../libs/aes_lib.rs"]
mod aes_lib;

#[test]
fn test_aes(){
	let data = "hello world!";
	let password = "12345";
	let enc = aes_lib::encrypt(data.as_bytes(), password);
	println!("Encrypt: {}",enc.as_str());
	let dec_bytes =  aes_lib::decrypt(enc.as_str(),password).unwrap();
	let decrypt_string = core::str::from_utf8(&dec_bytes).unwrap(); 
	println!("Decrypt: {}", decrypt_string);
	assert_eq!(data, decrypt_string);
}
