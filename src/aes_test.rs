#[cfg(test)]

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt , KeyInit,
    generic_array::GenericArray,
};

#[test]
fn test_aes() {
    let key = GenericArray::from([0u8;16]);
    let mut block = GenericArray::from([42u8;16]);
    let cipher  = Aes128::new(&key);
    let block_copy = block.clone();
    cipher.encrypt_block(&mut block);
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);
}
