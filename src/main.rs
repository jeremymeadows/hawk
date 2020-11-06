use std::{env, io, thread};
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, Mutex};

const SUCCESS: [u8; 1] = [1];
const FAIL: [u8; 1] = [0];

fn main() -> io::Result<()> {
    let _args: Vec<String> = env::args().collect();
    let peers: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    let listener = TcpListener::bind("127.0.0.1:9000");
    match &listener {
        // if machine can create a listener then it is the host
        Ok(listener) => {
            println!("listening");
            let listener = listener.try_clone()?;
            let mx = peers.clone();

            // thread to connect other peers
            thread::spawn(move || {
                loop {
                    let (mut stream, addr) = listener.accept().unwrap();
                    let mut peers = mx.lock().unwrap();
                    stream.write(&SUCCESS).unwrap();

                    let mut buf = [0 as u8; 528];
                    stream.read(&mut buf).unwrap();
                    if buf[0] == SUCCESS[0] {
                        println!("> client connected: {}", addr);

                        peers.push(stream);
                        drop(peers);
                    } else {
                        stream.shutdown(Shutdown::Both);
                    }

                }
            });
            loop{
                let mut msg = String::new();
                std::io::stdin().read_line(&mut msg)?;

                let mx = peers.clone();
                let peers = mx.lock().unwrap();

                for mut stream in &*peers {
                    stream.write(msg.as_bytes())?;
                }
                drop(peers);
            }
        },
        Err(_) => {
            let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9000")?;
            let mut buf = [0 as u8; 512];
            stream.read(&mut buf)?;
            if buf[0] == SUCCESS[0] {
                stream.write(&SUCCESS)?;
                println!("connected");
            } else {
                stream.shutdown(Shutdown::Both);
            }

            loop{
                let len = stream.read(&mut buf)?;
                if len > 0 {
                    println!("> {}", String::from_utf8(buf[0..len].to_vec()).unwrap());
                }
            }
        }
    }

    Ok(())
}
