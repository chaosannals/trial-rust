

pub struct LogPacker {
    key: [u8;32],
    pass: [u8;32],
}

impl LogPacker {
    pub fn new(key: &[u8;32], pass: &[u8;32]) -> LogPacker {
        let mut rkey = [0u8; 32];
        let mut rpass = [0u8; 32];
        rkey.copy_from_slice(key);
        rpass.copy_from_slice(pass);
        LogPacker { key: rkey, pass: rpass }
    }

    pub fn log_pack<'a>(&self, data : &'a [u8]) -> &'a [u8] {
        data
    }
    pub fn log_unpack<'a>(&self, data : &'a [u8]) -> &'a [u8] {
        data
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
