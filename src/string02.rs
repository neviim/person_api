use std::io::{self, Read};

fn main() {
    // String Dinamica
    let mut s = String::new();
    println!("Digite um texto");

    io::stdin().read_line(&mut s).expect("Erro reading console");

    println!("Foi digitado {s}");
}
