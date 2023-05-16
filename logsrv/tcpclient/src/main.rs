use std::{
    io,
    str,
    io::{
        Write,
        Read,
        Result,
    },
};
use std::net::{TcpStream};
use aes_gcm_siv::{
    Error,
    aead::{AeadInPlace, KeyInit, OsRng, Aead},
    Aes256GcmSiv, Nonce
};
use generic_array::{
    GenericArray,
    ArrayLength,
};

fn main() -> Result<()> {
    // let key  = Aes256GcmSiv::generate_key(&mut OsRng);
    let pass = b"pass";
    let mut keybuf = [0u8; 32];
    keybuf[0..pass.len()].copy_from_slice(&pass[..]);
    let key = GenericArray::from_slice(&keybuf);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let mut stream = TcpStream::connect("127.0.0.1:33333")?;
    let mut buf = [0u8; 128];
    loop {
        io::stdout().write_all(b"send: ")?;
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; // 空输入会导致接收方读不到。TODO 禁止空输入
        let d = input.trim().as_bytes();
        if d.len() > 0 {
            // let mut buffer: Vec<u8> = Vec::new();
            // buffer.extend_from_slice(&d);
            // cipher.encrypt_in_place(nonce, b"", &mut buffer)?;

            // 这个算法用了很奇怪的 Result ，和一般的 Result 不兼容，还要自己实现 Error
            // 直接 expect 处理了。 TODO 错误处理
            let ciphertext = cipher.encrypt(nonce, d).expect("encrypt error");
            stream.write(&ciphertext)?;

            // stream.write(&d)?;
            let n = stream.read(&mut buf[0..128])?;
            io::stdout().write_all(&buf[0..n])?;
        } else {
            io::stdout().write_all(b"empty")?;
        }
    }
}
