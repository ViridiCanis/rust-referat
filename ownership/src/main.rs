fn main() {
    // jeder nicht-primitiver Wert hat einen Besitzer (owner)
    // primitive Werte werden kopiert, statt bewegt
    let var = String::from("Hello World!");
    println!("(1) {}", var);


    // der Besitzer kann sich im Laufe des Programmes sich verändern
    let var2 = var;
    // println!("(2) {}", var); // Fehler: Variable wurde bewegt und gehört nicht mehr zu `var`
    println!("(2) {}", var2);


    // dies gilt auch für Funktionen
    let string = neuer_string();
    // `var` von `nimmt_besitzt` ist nun der Besitzer
    nimmt_besitzt(string);
    // println!("(3) {}", string); // Fehler


    // um die Werte nicht wegzugeben
    // kann man die stattdessen leihen (borrow) mit &
    // Achtung: der Compiler (eher "Borrow Checker") achtet darauf das die
    //     Referezen nicht länger als der eigentliche Wert leben können
    let string = String::from("Hello World!");
    let geliehender_string = &string;
    println!("(4) {}", string);
    println!("(4) {}", geliehender_string);

    // das gleiche mit Funktionen
    geliehen(&string);

    // und das kann auch erweitert werden auf Veränderbarkeit
    let mut gesamt = 0;
    let array = [1, 24, 32, 12, 3];
    // let ges_mut = &mut gesamt;
    addiere_länge(&mut gesamt, &array);
    // println!("{}", ges_mut);
}

fn neuer_string() -> String {
    String::from("Ich bin neu")
}

fn nimmt_besitzt(var: String) {
    println!("in Funktion nimmt_besitzt {}", var);
    // wird hier fallen gelassen am Ende der Funktion
    // implizites drop(var)
}

// &String kann automatisch in &str umgewandelt werden
// weshalb man eher &str verwenden sollte
// &str: "Hello"
// String: String::from("Hello")
fn geliehen(var: &str) {
    println!("in Funktion geliehen {}", var);
}

fn addiere_länge(gesamt: &mut usize, array: &[i32]) {
    // * um den Wert auf den von der Referenz gezeigt wird zu verändern
    *gesamt += array.len();
}

// file:///mnt/d/Documents/repositories/rust-referat/structs_enums/src/main.rs
