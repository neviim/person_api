#![allow(unused, dead_code)]
fn main() {
  // Memoria Stack
  let a: i32 = 1;    // é copy por padrão
  let b = a;
  let c = &b;
  println!("O valor de A é {}", a);
  println!("O valor de B é {}", b);
  // Por referencia
  println!("O valor de C é {}", *c);

  // Memoria Heap
  let aa = String::from("Jads"); // não copy por padrão
  // let bb = aa; // aqui o valor de aa esta sendo movido para bb
  // println!("O valor de aa é {}", aa); // o borrowed não permite passar este codigo

  let bb = &aa; // aqui o valor esta sendo emprestado
  println!("O valor de bb é {}", bb);

}

// Regra de Ownership em Rust
// cada valor tem um dono em (owner)
// Só pode ter um único dono
// Quando o dono sai de escopo o valor é limpo
// A posse (ownership) pode ser movida a outro dono