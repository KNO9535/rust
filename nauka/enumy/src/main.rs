enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn trasa(ip_kind: IpAddrKind){
    match ip_kind {
        IpAddrKind::V4(ip) => {
            println!("To jest adres IPv4: {}", ip);
        }
        IpAddrKind::V6(ip) => {
            println!("To jest adres Ipv6: {}", ip);
        }
    }
}

enum Message {
    Quit,                       
    Move { x: i32, y: i32 },    
    Write(String),              
    ChangeColor(i32, i32, i32), 
}

impl Message{
    fn call(&self){
        println!("wywolano metode call() dla {:?}", self);
    }
}

enum Option<T>{
    Some(T),
    None,
}

fn main() {
    let cztery = IpAddrKind::V4(127, 0, 0, 1);
    let szesc = IpAddrKind::V6("::1".to_string());

    let m = Message::Write(String::from("Wiadomosc"));
    m.call();

    let jakas_liczba = Some(5);
    let jakis_tekst = Some("Tekst");
    let brak = None::<i32>;

    trasa(cztery);
    trasa(szesc);
}
