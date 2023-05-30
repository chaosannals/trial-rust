use std::io::prelude::*;
use std::io::{stdout};
use std::sync::mpsc::Sender;
use std::thread::{spawn};
use std::error;
use std::result;
use std::str;
use std::sync::{Mutex, mpsc};
use std::net::{TcpListener, TcpStream};
use aes_gcm_siv::{
    Error,
    aead::{AeadInPlace, KeyInit, OsRng, Aead},
    Aes256GcmSiv, Nonce
};
use generic_array::{
    GenericArray,
    ArrayLength,
};

fn handle_loop(s: &mut TcpStream, sender: &Sender<String>) -> result::Result<(), Box<dyn error::Error>> {
    let pass = b"pass";
    let mut keybuf = [0u8; 32];
    keybuf[0..pass.len()].copy_from_slice(&pass[..]);
    let key = GenericArray::from_slice(&keybuf);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    loop {
        let r : result::Result<(), Box<dyn error::Error>> = {
            let b = &mut [0; 32];
            let n = s.read(b)?;

            // TODO 这个的 Result 很奇怪，实现不知道放哪个包了。
            let text = cipher.decrypt(nonce, &b[0..n]).expect("decrypt error");
            s.write(&text)?;

            if let Ok(s) = str::from_utf8(&text) {
                sender.send(s.to_string())?;
            }
            sender.send("Ok".to_string())?;
            Ok(())
        };
    }
}

fn main() -> std::io::Result<()> {
    // let (error_output,error_input) = mpsc::channel();
    let (output, input) = mpsc::channel();
    
    spawn(move || {
        let address = "0.0.0.0:33333";
        let listener = TcpListener::bind(address).expect("bind error");
        // listener.set_nonblocking(true).expect("Cannot set non-blocking");

        for stream in listener.incoming() {
            // 此处是同步的，标准库没有异步库，需要使用第三方。
            // 此处读写最好另外起线程或者协程，防止阻塞。
            // let err_sender = error_output.clone();
            let sender = output.clone();
            match stream {
                Ok(mut s) => {
                    spawn(move || {
                        if let Err(e) = handle_loop(&mut s, &sender) {
                            // err_sender.send(e.to_string()).expect("send error");
                            sender.send(e.to_string()).expect("send error");
                        }
                    });
                }
                Err(e) => {
                    //error_output.send(format!("error: {:?}", e)).expect("send error");
                    output.send(format!("error: {:?}", e)).expect("send error");
                }
            }
        }
    });
    println!("start server.");
    loop {
        let out_msg: String = input.recv().expect("input error");
        println!("output: {}", out_msg);
        //let err_msg: String = error_input.recv().expect("err input error");
        //println!("error: {}", err_msg);
    }
}
