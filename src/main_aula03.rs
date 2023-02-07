use std::primitive;

fn main() {
  // primitivo escalares cada variavel armazenada armazena um unico valor

  // inteiro
  let x = 5;
  let y = 300_200_100;
  let h = 0xff;
  let o = 0o77;
  let b = 0b1111_0000;
  let by = b'A';
  // flouts
  let x: f64 = 42.1;
  // boleano
  let x: bool = false;
  // symbolos, aceita emojo
  let letra: char = 'a';

  // tupla
  let nm: (i32, i32, i32) = (1, 2, 3);
  println!("{:?}", nm );
  println!("{:?}", nm.2 );

  // desestruturar tupla
  let (a, b, c) = nm;
  println!("{:?}", c);

  // deixar a tupla mutavel
  let mut numero = (1, 2, 3);
  numero.0 = 50;
  println!("{:?}", numero);
  // tambem pode ser utilizado assim
  numero = (4, 5, 6);
  println!("{:?}", numero);
  // tbm pode misturar tipos
  let numero_01 = (4, false, 7.3);
  println!("{:?}", numero_01);

  // usando array, só pode ser numeros iguais
  let num: [i32;4] = [1, 2, 3, 4];
  println!("{:?}", num);
  // como acessar array, usando chave
  let num = [1.1, 2.2, 3.3, 4.4];
  println!("{:?}", num[3]);
  // tbm pode ser mutavel
  let mut num = [1.1, 2.2, 3.3];
  num[0] = 50.5;
  println!("{:?}", num);
  // slice
  println!("{:?}", &num[1..2]);
  println!("{:?}", &num[1..]);
  println!("{:?}", &num[..2]);

}