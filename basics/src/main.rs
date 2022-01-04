#![allow(unused)]

// die Funktion main ist der Startpunkt eines Programms
// Funktionen werden mit fn definiert
fn main() {
    // Variable definieren: let Name: Typ = Wert;
    // Typ ist optional, wenn der Compiler es selbst herleiten kann
    let a = 10;
    let b: u64 = 123;


    // Variabeln mit gleichen Namen werden überschriben: Shadowing
    let a = 15;
    // a += 20; // Fehler: Variabeln sind standardmäßig nicht veränderbar


    // mut muss hinzugefügt werden, um eine Variabel veränderbar (mutable) zu machen
    let mut a = 12;
    println!("(1) a = {}", a);
    a += 3;
    println!("(2) a = {}", a);


    let b = addiere(a, 4);
    println!("(3) a = {}", a);
    println!("(3) b = {}", b);
}

// Argumente in Klammern mit Typen
// Rückgabetyp nach ->
fn addiere(x: i32, y: i32) -> i32 {
    // wenn die letzte Aussage kein ; hat
    // ist es der Rückgabewert
    x + y
}





// file:///mnt/d/Documents/repositories/rust-referat/ownership/src/main.rs
