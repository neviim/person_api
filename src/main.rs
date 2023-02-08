#![allow(unused, dead_code)]
// usando memoria static
fn say_hello_memoria_static(text: &str) {
  println!("Hello, {text}");
}

fn say_goodbay_memoria_static(text: &str) {
  println!("Goodbay, {text}");
}

// usando memoria heap
fn say_hello_memoria_reap(text: &String) {
  println!("Hello, {text}");
}

fn say_goodbay_memoria_heap(text: &String) {
  println!("Goodbay, {text}");
}

// alterando o valor da variavel na memoria
fn to_uppercase(text: &mut String) {
  *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
  *text = format!("FOO_{text}")
}

fn main() {
  // usando memoria static
  let name_static = "Jads"; // &str - static - tipo Copy
  say_hello_memoria_static(name_static);
  say_goodbay_memoria_static(name_static);

  // usando memoria Heap
  let name_heap = "Jads".to_string();
  // say_hello_memoria_reap(name_heap.clone());  // opção um mais não é aconselhado usar isso
  say_hello_memoria_reap(&name_heap);            // emprestando - (Borrow) - é só uma referencia na memoria
  say_goodbay_memoria_static(&name_heap);

  // e se eu tiver mudando o valor de memoria
  let mut name = "Jads".to_string();
  to_uppercase(&mut name);
  add_prefix(&mut name);
  println!("{name}");

}