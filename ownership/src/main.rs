fn main() {
    // jeder nicht-primitiver Wert hat einen Besitzer (owner)
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
    let string = String::from("Hello World!");
    let geliehender_string = &string;
    println!("(4) {}", string);
    println!("(4) {}", geliehender_string);

    // das gleiche mit Funktionen
    geliehen(&string);


    // und das kann auch erweitert werden auf Veränderbarkeit
    let mut gesamt = 0;
    let array = [1, 24, 32, 12, 3];
    addiere_länge(&mut gesamt, &array);
}

fn neuer_string() -> String {
    String::from("Ich bin neu")
}

fn nimmt_besitzt(var: String) {
    println!("in Funktion nimmt_besitzt {}", var);
    // wird hier fallen gelassen am Ende der Funktion
    // implizites drop(var)
}

fn geliehen(var: &String) {
    println!("in Funktion geliehen {}", var);
}

fn addiere_länge(gesamt: &mut usize, array: &[i32]) {
    // * um den Wert auf den von der Referenz gezeigt wird zu verändern
    *gesamt += array.len();
}