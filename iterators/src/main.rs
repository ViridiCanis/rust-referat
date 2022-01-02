fn main() {
    // iteratoren sind structs die das Trait Iterator implentieren
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // verschiedene Container wie Vec oder slice/array
    // haben eine Methode iter, die einen Iter gibt,
    // welches Iterator implentiert
    let res = arr.iter()
    // bei diesen kann funktionale Paradigmen anwenden
        .filter(|&&x| x & 2 == 0) // alle geraden Zahlen
        .take(5) // die ersten 5 davon
        .map(|x| x * 2) // gedoppelt
        .collect::<Vec<_>>(); // als ein Vec

    println!("{:?}", res);

    // man kann durch Iteratoren mit einem for-loop iterieren
    // oder manuell mit der Methode next
    let mut iter = arr.iter();
    while let Some(val) = iter.next() {
        println!("while over iterator: {}", val);
    }

    // ruft selbst iter auf
    for val in arr {
        println!("for over slice: {}", val);
    }

    // for über Iterator
    for val in arr.iter().map(|x| x * 2) {
        println!("for over Map: {}", val);
    }

    for i in Counter::new().step_by(3).take(20) {
        println!("jede dritte Zahl 20-mal {}", i);
    }
}

// eigene iteratoren müssen nur next implementieren und der Rest wie map, take und filter
// kommt zusätzlich
struct Counter {
    count: i32
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
