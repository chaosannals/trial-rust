use std::io::prelude::*;
use std::io::{stdout};
use std::thread::{spawn};
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use aes_gcm_siv::{
    aead::{AeadInPlace, KeyInit, OsRng, Aead},
    Aes256GcmSiv, Nonce
};
use generic_array::{
    GenericArray,
    ArrayLength,
};


fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:33333";
    let listener = TcpListener::bind(address)?;
    // listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        // 此处是同步的，标准库没有异步库，需要使用第三方。
        // 此处读写最好另外起线程或者协程，防止阻塞。
        match stream {
            Ok(mut s) => {
                // let handle = spawn(move || {
                spawn(move || {
                    let pass = b"pass";
                    let mut keybuf = [0u8; 32];
                    keybuf[0..pass.len()].copy_from_slice(&pass[..]);
                    let key = GenericArray::from_slice(&keybuf);
                    let cipher = Aes256GcmSiv::new(&key);
                    let nonce = Nonce::from_slice(b"unique nonce");

                    loop {
                        let b = &mut [0; 32];
                        let n = s.read(b).expect("read error");

                        let text = cipher.decrypt(nonce, &b[0..n]).expect("decrypt error");
                        s.write(&text).expect("read error");

                        // stdout().lock().write_all(&text).expect("write error"); // std 写入无效
                        // stdout().lock().flush();
                    }
                });
                // handle.join().expect("join error");
            }
            Err(e) => {
                stdout().write_all(format!("error: {:?}", e).as_bytes())?;
            }
        }
    }
    Ok(())
}
