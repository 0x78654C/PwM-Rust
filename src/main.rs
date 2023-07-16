use std::env;
//use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;

use argon2::{self, Config, ThreadMode, Variant, Version};
use base64::encode;

//type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
//type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;



const HELP_MESSAGE: &str = "\
PwM Copyright @ 2020-2022 0x078654c
PwM - A simple password manager to store localy the authentification data encrypted for a application using Rijndael AES-256 and Argon2 for password hash.
Contact: xcoding.dev@gmail.com



[x] Usage of Password Manager commands:
      -h       : Display this message.
      -createv : Create a new vault.
      -delv    : Deletes an existing vault.
      -listv   : Displays the current vaults.
      -addapp  : Adds a new application to vault.
      -dela    : Deletes an existing application in a vault.
      -updatea : Updates account's password for an application in a vault.
      -lista   : Displays the existing applications in a vault.
";

/*[test]
fn test_aes() {
    let key : GenericArray::from([0u8;16]);
    let mut block = GenericArray::from([42u8;16]);
    let cipher  = Aes128::new(&key);
    let block_copy = block.clone();
    cipher.encrypt_block(&mut block);
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);
}*/
fn aes_ecnrypt(){

	let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
	let mut message = String::from("Hello world!");
	let mut mykey =String::from("000102030405060708090A0B0C0D0E0F");
	let args: Vec<String> = env::args().collect();

	if args.len() >1 {
		message = args[1].clone();
	}
 
	if args.len() >2 {
		mykey = args[2].clone();
	}

	println!("Message: {}",message);
	println!("Key: {}",mykey);
	println!("IV: f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
	let plaintext=message.as_bytes();
	let key = hex::decode(mykey).ZZexpect("Decoding failed");
	let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
	let pos = plaintext.len();
	let mut buffer = [0u8; 128];
	buffer[..pos].copy_from_slice(plaintext);
	let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
	println!("\nCiphertext: {:?}",hex::encode(ciphertext));
	let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
	let mut buf = ciphertext.to_vec();
	let mut decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
	let decrypted_message =str::from_utf8(decrypted_chipertext).unwrap();
	println!("\nCiphertext: {:?}",decrypted_message);
	assert_eq!(decrypted_message, message);
}

// Return arogn hash
fn argon_hash_genrator(password: &String)->String {
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
    //println!{"{}", pass_substring};
    //println!("{:?}", salt);
    //let hash_enc = encode(hash);
    //let matches = argon2::verify_encoded(&hash, password).unwrap();
    return hash;
}


struct Globals {
    number: i32
}

impl Globals{
    fn up(&mut self, num: i32){
        self.number += num;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
       
    let mut VAULT_DIR:String = "".to_string();
    let arg1 = &args[1];
    println!("{}", arg1);
}

// Check maximum  of tries. used in while loops for exit them at a certain count.
fn check_max_tries()->bool{
    let mut TRIES = Globals{number:0};
    if TRIES.number >= 3 {
        println!("You have exceeded the number of tries!");
        TRIES.up(TRIES.number*-1);
        return true;
    }
    return false;
}
