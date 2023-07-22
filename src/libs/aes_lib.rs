use std::error::Error;
use std::{io, str};
use std::io::ErrorKind;
use std::iter::repeat;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes_gcm::AesGcm;

/// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
/// is the data returned from encrypt(). This function splits the data, removes
/// the hex encoding, and returns each as a list of bytes.
fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').into_iter().collect();

    if split.len() != 3 {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv_res = hex::decode(split[0]);
    if iv_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv = iv_res.unwrap();

    let data_res = hex::decode(split[1]);
    if data_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let data = data_res.unwrap();

    let mac_res = hex::decode(split[2]);
    if mac_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let mac = mac_res.unwrap();

    Ok((iv, data, mac))
}

/// gets a valid key. This must be exactly 16 bytes. if less than 16 bytes, it will be padded with 0.
/// If more than 16 bytes, it will be truncated
fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    if bytes.len() < 16 {
        for j in 0..(16 - bytes.len()) {
            bytes.push(0x00);
        }
    } else if bytes.len() > 16 {
        bytes = bytes[0..16].to_vec();
    }

    bytes
}

///Decryption using AES-GCM 256
///iv_data_mac is a string that contains the iv/nonce, data, and mac values. All these values
/// must be hex encoded, and separated by "/" i.e. [hex(iv)/hex(data)/hex(mac)]. This function decodes
/// the values. key (or password) is the raw (not hex encoded) password
pub fn decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
    let key = get_valid_key(key);

    let key_size = crypto::aes::KeySize::KeySize256;

    // I don't use the aad for verification. aad isn't encrypted anyway, so it's just specified
    // as &[].
    let mut decipher = AesGcm::new(key_size, &key, &iv, &[]);

    // create a list where the decoded data will be saved. dst is transformed in place. It must be exactly the same
    // size as the encrypted data
    let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
    let result = decipher.decrypt(&data, &mut dst, &mac);

    if result { println!("Successful decryption"); }
    println!("\nDecrypted {}", str::from_utf8(&dst).unwrap());

    Ok(dst)
}

/// Creates an initial vector (iv). This is also called a nonce
fn get_iv(size: usize) -> Vec<u8> {
    let mut iv = vec![];
    for j in 0..size {
        let r = rand::random();
        iv.push(r);
    }

    iv
}

///encrypt "data" using "password" as the password
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)
pub fn encrypt(data: &[u8], password: &str) -> String {
    let key_size = crypto::aes::KeySize::KeySize256;

    //pad or truncate the key if necessary
    let valid_key = get_valid_key(password);
    let iv = get_iv(12); //initial vector (iv), also called a nonce
    let mut cipher = AesGcm::new(key_size, &valid_key, &iv, &[]);

    //create a vec of data.len 0's. This is where the encrypted data will be saved.
    //the encryption is performed in-place, so this vector of 0's will be converted
    //to the encrypted data
    let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();

    //create a vec of 16 0's. This is for the mac. This library calls it a "tag", but it's really
    // the mac address. This vector will be modified in place, just like the "encrypted" vector
    // above
    let mut mac: Vec<u8> = repeat(0).take(16).collect();

    //encrypt data, put it into "encrypted"
    cipher.encrypt(data, &mut encrypted, &mut mac[..]);

    //create the output string that contains the nonce, cipher text, and mac
    let hex_iv = hex::encode(iv);
    let hex_cipher = hex::encode(encrypted);
    let hex_mac = hex::encode(mac);
    let output = format!("{}/{}/{}", hex_iv, hex_cipher, hex_mac);

    output
}
