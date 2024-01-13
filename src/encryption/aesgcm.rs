use aes_gcm::{KeyInit, Aes256Gcm, Key, aead::{OsRng, Aead, Nonce}, AeadCore};

pub struct Secure {
    buf: Vec<u8>,
    nonce: Vec<u8>,
}

impl Secure {
    pub fn new(plaintext: &[u8], key: &[u8]) -> Self {
        let aes_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(&aes_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap();

        Self {
            nonce: nonce.to_vec(),
            buf: ciphertext
        }
    }

    pub fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        let aes_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(&aes_key);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&self.nonce);
        let result = cipher.decrypt(&nonce, self.buf.as_ref());

        match result {
            Ok(plaintext) => return Some(plaintext),
            Err(_) => return None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Secure;

    #[test]
    fn roundtrip() {
        let key = "secret key";
        let message = b"Hello, world!";

        let secure = Secure::new(message, key.as_bytes());
        let plaintext = secure.read(key.as_bytes()).unwrap();

        // assert_eq!(message.len(), plaintext.as_slice().len());
        assert_eq!(message, plaintext.as_slice());
    }
}