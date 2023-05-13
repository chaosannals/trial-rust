use aes::Aes256;
use aes::cipher::{
    KeyInit,
    BlockEncrypt,
    BlockDecrypt,
    generic_array::{
        GenericArray,
    }
};
// TODO crypt
// use crypto::aes_gcm::AesGcm;

pub struct LogPacker {
    key: [u8;32],
    pass: [u8;32],
    cipher: Aes256,
}

impl LogPacker {
    pub fn new(key: &[u8;32], pass: &[u8;32]) -> LogPacker {
        let mut rkey = [0u8; 32];
        let mut rpass = [0u8; 32];
        rkey.copy_from_slice(key);
        rpass.copy_from_slice(pass);
        let gkey = GenericArray::from(rkey);
        LogPacker {
            key: rkey,
            pass: rpass,
            cipher: Aes256::new(&gkey),
        }
    }

    // pub fn log_pack<'a>(&self, data : &'a [u8]) -> Vec<u8> {
    //     let mut result = vec![];
    //     let c = (data.len() as f64 / 32.0).ceil() as i32;
    //     let mut buffer = [0u8; 32];
    //     for i in 0..c {
    //         buffer[0..data.len()].copy_from_slice(&data);
    //         println!("buffer length: {:?}", &buffer.len());
    //         let mut block = GenericArray::clone_from_slice(&buffer);
    //         self.cipher.encrypt_block(&mut block);
    //         result.extend_from_slice(&block);
    //     }
    //     result
    // }

    pub fn log_pack<'a>(&self, data : &'a [u8]) -> Vec<u8> {
        let mut result = vec![];
        result.extend_from_slice(&data);
        result
    }

    pub fn log_unpack<'a>(&self, data : &'a [u8]) -> Vec<u8> {
        let n = data.len();
        println!("1 {:?}", n);
        let mut block = GenericArray::clone_from_slice(&data);
        println!("2");
        self.cipher.decrypt_block(&mut block);
        println!("3");
        block.as_slice().to_vec()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
