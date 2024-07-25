pub struct VigenereCipher {
    key: Vec<u8>,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        VigenereCipher {
            key: key.as_bytes().to_vec(),
        }
    }

    pub fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        self.apply_cipher(plaintext, true)
    }

    pub fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        self.apply_cipher(ciphertext, false)
    }

    fn apply_cipher(&self, data: Vec<u8>, encrypt: bool) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len());
        let key_bytes = &self.key;
        let key_len = key_bytes.len();

        for (i, &data_byte) in data.iter().enumerate() {
            let key_byte = key_bytes[i % key_len];
            let shift = key_byte;
            let base = if encrypt {
                data_byte.wrapping_add(shift)
            } else {
                data_byte.wrapping_sub(shift)
            };
            result.push(base);
        }
        result
    }
}


