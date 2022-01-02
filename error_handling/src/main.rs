#![allow(unused)]

use std::fs;
use std::io::stdin;

fn main() {
    // zwei Arten von Fehlern:
    //   behebbare
    //   nicht behebbare

    // nicht behebbar

    // panic!("fataler Fehler");

    // fs::read("nicht_existierende_datei").unwrap();


    // behebbar

    let mut num: i32 = 0;
    loop {
        // neuer leerer String, da read_line am Ende anhängt
        let mut input = String::new();
        print!("Zahl: ");
        stdin().read_line(&mut input).expect("failed reading from stdin");

        // trim, weil \n im String ist
        match input.trim().parse() {
            Ok(val) => {
                num = val;
                break;
            },
            Err(err) => println!("Bitte gebe eine gültige Zahl ein! ({}): {}", err, input)
        }
    }

    println!("gewählte Zahl: {}", num);
}
