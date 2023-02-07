use std::io;

fn main() {
    // String Dinamica
    let mut s = String::new();
    println!("Digite um texto");
    println!("{}", "-".repeat(20));
    println!("{:-^20}", "JADS");

    io::stdin().read_line(&mut s).expect("Erro reading console");

    println!();  // Э
    println!("Foi digitado {s}");
    println!("Qauntidade de letras {}", s.trim().len());
    println!("Qauntidade de letras {}", s.trim().chars().count());
    println!("Tudo em maiusculo {}", s.to_uppercase());
    println!("Trocando a letras {}", s.replace("a", "e"));
}