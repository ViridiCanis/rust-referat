use std::io::prelude::*;
use std::io::ErrorKind;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::{TcpListener, TcpStream};

// https://lib.rs/crates/routerify

// da std nichts für Zeitformatierung macht (weil Zeit kompliziert ist)
use chrono::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed binding to port 8000");
    listener.set_nonblocking(true).expect("cannot set nonblocking");

    println!("running on port 8000");

    let mut threads = Vec::new();
    let running = Arc::new(AtomicBool::new(true));

    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                println!("new connection");

                let running = Arc::clone(&running);
                threads.push(thread::spawn(move || {
                    handle_conn(conn, running);
                }));
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock && !running.load(Ordering::Acquire) {
                    break;
                }
            }
        }
    }

    println!("shutting down");

    for t in threads {
        if let Err(_) = t.join() {
            eprintln!("a thread failed at some point");
        }
    }

    println!("finished shutting down");
}

fn handle_conn(mut conn: TcpStream, running: Arc<AtomicBool>) {
    let response = format!(
        concat!(
            "Hello {}\r\n",
            "connected at {}\r\n"
        ),
        conn.peer_addr().unwrap(),
        Local::now()
    );

    conn.write(response.as_bytes()).unwrap();

    loop {
        let mut buf = [0u8; 1024]; // 1 kiB Buffer
        conn.read(&mut buf).unwrap();
        let msg = String::from_utf8(buf.to_vec()).unwrap();

        if msg.starts_with("disconnect") {
            let response = "Goodbye\r\n";

            conn.write(response.as_bytes()).unwrap();
            return;
        } else if msg.starts_with("server_shutdown") {
            running.store(false, Ordering::Release);

            let response = concat!(
                "Goodbye\r\n",
                "Shutting down server\r\n"
            );

            conn.write(response.as_bytes()).unwrap();
            return;
        } else {
            conn.write(msg.as_bytes()).unwrap();
        }
    }
}
