#![allow(unused)]

#[derive(Debug)]
enum Vergleich {
    Größer,
    Gleich,
    Kleiner
}

#[derive(Debug)]
struct Rechteck {
    breite: u32,
    höhe: u32
}

impl Rechteck {
    fn new(breite: u32, höhe: u32) -> Self {
        Self {
            breite,
            höhe
        }
    }

    fn quadrat(seitenlänge: u32) -> Self {
        Self {
            breite: seitenlänge,
            höhe: seitenlänge
        }
    }

    fn fläche(&self) -> u32 {
        self.breite * self.höhe
    }

    fn kann_halten(&self, anderes: &Rechteck) -> bool {
        self.breite > anderes.breite &&
        self.höhe > anderes.höhe
    }

    fn vergleich_fläche(&self, anderes: &Rechteck) -> Vergleich {
        let eigene_fläche = self.fläche();
        let anderes_fläche = anderes.fläche();

        if eigene_fläche < anderes_fläche {
            Vergleich::Kleiner
        } else if eigene_fläche == anderes_fläche {
            Vergleich::Gleich
        } else {
            Vergleich::Größer
        }
    }
}

// Enums können auch Daten besitzen
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// und eigene Methoden haben
impl IpAddr {
    fn new_v4(part1: u8, part2: u8, part3: u8, part4: u8) -> IpAddr {
        IpAddr::V4(part1, part2, part3, part4)
    }
}

// Was es beinhaltet kann deutlich variieren
enum Nachricht {
    Quit,
    Bewege { x: i32, y: i32 },
    Schreibe(String),
    VeränderFarbe(i32, i32, i32)
}

fn main() {
    // let rechteck = Rechteck {
    //     breite: 10,
    //     höhe: 20
    // };
    // oder da wir new definiert haben
    let rechteck = Rechteck::new(10, 20);
    println!("Fläche: {}", rechteck.fläche());

    let quadrat = Rechteck::quadrat(5);
    println!("Kann das Quadrat halten: {}", rechteck.kann_halten(&quadrat));

    let zweites = Rechteck::new(5, 50);
    println!("erstes Rechteck ist {:?} dem Zweiten", rechteck.vergleich_fläche(&zweites));


    let ipv4_addr = IpAddr::new_v4(192, 178, 0, 1);


    let nachricht = Nachricht::Bewege { x: 14, y: 45 };

    // Pattern Matching
    // dies muss alle Option verarbeiten, sonst gibt es eine Fehlermeldung vom Compiler
    match nachricht {
        Nachricht::Quit => {/* beende das Programm */},

        Nachricht::Bewege { x, y: 0 } => println!("bewegt zu {} auf der x-Achse", x),

        Nachricht::Bewege { x, y } => println!("bewegt zu {} {}", x, y),

        // dieser Fall ist für alle Nachrichten, die nicht Quit oder Schreibe sind
        // _ als Namen einer Variabel ignoriert den zugewisenden Wert
        // z.B. let _ = 10; macht nichts
        _ => {/* ignoriere alle anderen Nachrichten */}
    }

    // um nur eine Variante zu behandeln
    if let Nachricht::Schreibe(text) = nachricht {
        println!("Nachricht: {}", text);
    }


    // per Enums wird Option definiert
    // Option ersetzt null von anderen Programmiersprachen
    let viell_var: Option<u32> = Some(13);
    let viell_var2: Option<u32> = None;

    // wenn man sich sicher ist das es einen Wert hat
    // kann man unwrap oder ähnliches nutzen, was das Programm abstürzen lässt
    // wenn es keinen Wert hat
    println!("Wert in viell_var: {}", viell_var.unwrap());

    // Fehler: "panicked at 'called Option::unwrap() on a `None` value'"
    println!("{}", viell_var2.unwrap());
}
