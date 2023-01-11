use hex_literal::hex;

use aes_gcm::aead::{generic_array::GenericArray, Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Aes256Gcm};

use std::str;



#[test]
fn test_encrypt_decrypt() {
    let key = hex!("11754cd72aec309bf52f7687212e8957");
    let nonce = hex!("3c819d9a9bed087615030b65");
    let plaintext =  b"some data";
    let aad = b"";

    let key = GenericArray::from_slice(&key);
    let nonce = GenericArray::from_slice(&nonce);
    let payload = Payload {
        msg: plaintext,
        aad,
    };

    let cipher = Aes128Gcm::new(key);
    let ciphertext = cipher.encrypt(nonce, payload).unwrap();

    let plaintext = cipher.decrypt(nonce, Payload {
      msg: &ciphertext,
      aad
    }).unwrap();
    
    println!("{:?}", str::from_utf8(&plaintext).unwrap())
}