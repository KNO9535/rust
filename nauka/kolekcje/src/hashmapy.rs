use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Wynik Blue: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Stringi są przenoszone, a typy implementujące trait Copy są kopiowane
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name i field_value są tutaj nieważne! (błąd kompilacji przy próbie użycia)

    // Nadpisywanie: dodanie wartości pod istniejący klucz zastępuje starą wartość
    scores.insert(String::from("Red"), 67);
    scores.insert(String::from("Red"), 33); // nadpisuje 67
    println!("{scores:?}"); // Red ma teraz 33 (obok Blue: 10 i Yellow: 50)

    // Wstawianie tylko jeśli klucz nie istnieje
    // Sprawdza "Green", nie ma go, więc dodaje 50
    scores.entry(String::from("Green")).or_insert(50);
    // Sprawdza "Blue", już istnieje, więc nic nie robi
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}"); // {"Blue": 10, "Yellow": 50, "Red": 33, "Green": 50} (kolejność losowa)

    // Aktualizacja na podstawie starej wartości
    // or_insert zwraca mut ref &mut V do wartości ukrytej pod kluczem
    // można to wykorzystać do liczenia wystąpień słów w tekście
    let text = "hello world wonderful world";
    let mut mapa = HashMap::new();

    for word in text.split_whitespace() {
        let count = mapa.entry(word).or_insert(0);
        *count += 1; // dereferencja, żeby zmodyfikować wartość w HashMapie
    }

    println!("{mapa:?}"); // {"world": 2, "hello": 1, "wonderful": 1} (kolejność losowa)
}