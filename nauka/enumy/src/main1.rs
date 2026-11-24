#[derive(Debug)]
enum StanUsa{
    Alabama,
    Alaska,
}

enum Moneta{
    Penny,
    Nickel,
    Dime,
    Quarter(StanUsa), //trzyma informacje o stanie
}

fn wartosc_w_centach(moneta: Moneta) -> u8{
    match moneta{
        Moneta::Penny => {
            println!("To jest pens!");
            1 //zwracamy wartosc 1
        },
        Moneta::Nickel => 5,
        Moneta::Dime => 10,
        Moneta::Quarter(stan) => {
            println!("To 25-centowka ze stanu {:?}", stan);
            25
        },
    }
}

enum Option<T>{
    Some(T),
    None,
}

fn plus_jeden(x: Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i + 5),
        None => None
    }
}

fn opisz_monete(moneta: Moneta) -> Option<String> {
    // 1. Próbujemy wyciągnąć "stan". Jeśli moneta to NIE JEST Quarter...
    let Moneta::Quarter(stan) = moneta else{
        return None; //...przerywamy funkcje
    }

    //2. jestesmy na szczesliwej sciezce, stan jest teraz dostepny normalnie bez dodatkowych nawiasow
    if stan.istnial_w(1900){
        Some(format!("{:?} to stary stan!", stan))
    }else{
        Some(format!("{:?} to nowy stan!", stan))
    }
}


fn main(){
    let piec = Some(5);
    let szesc = plus_jeden(piec); // Zwróci Some(6)
    let puste = plus_jeden(None); // Zwróci None

    let rzut_kostka: i32 = 9;

    match rzut_kostka {
        3 => dodaj_kapelusz(),
        7 => usun_kapelusz(),
        inna_liczba => rusz_gracza(inna_liczba), // Łapie wszystko inne i przypisuje do zmiennej 'inna_liczba'
        // _ => () //kompletnie nice jak zlapie inna
    }

    let maksymalna_konfiguracja = Some(3u8);
    

    if let Some(max) = maksymalna_konfiguracja {
        println!("Maksymalna wartosc to {}", max);
    }else{
        licznik += 1;
    }

    let moneta = Moneta::Quarter(StanUsa::Alaska);
    let mut licznik = 0;

    if let Moneta::Quarter(stan) = moneta {
        println!("Stan usa to {:?}", stan);
    }else{
        licznik += 1;
    }
}