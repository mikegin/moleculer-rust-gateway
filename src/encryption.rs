use aes_gcm::aead::OsRng;
use aes_gcm::aead::generic_array::ArrayLength;
use hex_literal::hex;

use aes_gcm::aead::{generic_array::GenericArray, Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Nonce};

use std::str;

use uuid::{Bytes, Uuid};

use rand::{distributions::Alphanumeric, Rng};


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

#[test]
fn test_encrypt_decrypt_bld_tok() {
    // let key = Aes128Gcm::generate_key(&mut OsRng);
    // let nonce = Uuid::new_v4();
    // let plaintext =  b"some data";
    // let aad = b"";

    // let key = GenericArray::from_slice(&key);
    // // let nonce = GenericArray::from_slice(nonce.as_bytes());
    // let nonce = GenericArray::from_slice(nonce.as_bytes());
    // let payload = Payload {
    //     msg: plaintext,
    //     aad,
    // };

    // let cipher = Aes128Gcm::new(key);
    // let ciphertext = cipher.encrypt(nonce, payload).unwrap();

    // let plaintext = cipher.decrypt(nonce, Payload {
    //   msg: &ciphertext,
    //   aad
    // }).unwrap();
    
    // println!("{:?}", str::from_utf8(&plaintext).unwrap())








    let key = Aes128Gcm::generate_key(&mut OsRng);
    let cipher = Aes128Gcm::new(&key);
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(96 / 8)// 96 bit nonce requirement, 8 bits per char
        .map(char::from)
        .collect();
    let nonce = Nonce::from_slice(s.as_bytes());
    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref()).unwrap();
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");
}