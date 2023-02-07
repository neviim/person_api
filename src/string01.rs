fn main() {
  // String Slaise
  let nome: &str = "Jads";
  println!("{}", nome);

  // Heap String, String Dinamica
  let s: String = "Jads".to_string();
  println!("{s}");
  //
  let nome = ['J', 'a', 'd', 's'];
  let s = String::from_iter(nome);
  println!("{s}");
  //
  let s: String = "Jads".into();
  println!("{s}");

  // String owned (ha outa forma sera visto depois)
  let mut ss = String::from("Jads");
  println!("{ss}");
  //
  let mut ss = "Jads".to_owned();
  println!("{ss}");
}