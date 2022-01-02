use std::io::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};
use std::net::TcpListener;

// da std nichts für Zeitformatierung macht (weil Zeit kompliziert ist)
use chrono::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed binding to port 8000");
    println!("running on port 8000");
    let mut threads = Vec::new();
    let successful_conns = Arc::new(Mutex::new(0));

    while *successful_conns.lock().unwrap() < 10 {
        let conn = listener.accept();
        if let Ok((mut conn, peer)) = conn {
            println!("new connection");

            let counter = Arc::clone(&successful_conns);
            threads.push(thread::spawn(move || {
                let response = format!(
                    concat!(
                        "Hello {}\r\n",
                        "at {}\r\n"
                    ),
                    peer,
                    Local::now()
                );

                conn.write(response.as_bytes()).unwrap();

                let mut count = counter.lock().expect("counter Mutex was poisoned");
                *count += 1;
            }));
        }
    }

    println!("had 10 successful connections, shutting down");

    for t in threads {
        if let Err(_) = t.join() {
            eprintln!("a thread failed at some point");
        }
    }

    println!("finished shutting down");
}
