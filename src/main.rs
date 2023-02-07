fn say_hello(name: &str, color: &str) {
  println!("Ola! {name}, cor {color}" );
}

fn add_numbers(x: i32, y: i32) -> i32 {
  if x == 0 {
    return y;
  };
  x + y  // sempre a ultima expreção sera o return
         // e não tem ; no fim da expresão
}

fn convert_to_number(s: &str) -> i32 {
  s.parse().unwrap()
}

fn double(n: i32) -> i32 {
  n * 2
}

fn main() {
  say_hello("Jads", "Black");
  say_hello("Neviim", "REd");

  let y: () = {
    say_hello("Que isso:", "Branca");
    let x = 5;
  };

  println!("{:?}", y);

  let res = add_numbers(0, 9);
  println!("{res}");

  // demostra uso de função simples
  let input = "56 65 58 48 59 56 87 23";

  let result: Vec<i32> = input
    .split(' ')
    .map(convert_to_number)
    .map(double)
    .collect();

  println!("{:?}", result);

  // A estrutura acima pode ser aplicada desta forma
  let result: Vec<i32> = input
    .split(' ')
    .map(|s| s.parse::<i32>().unwrap())
    .map(|n| n * 2)
    .collect();

  println!("{:?}", result);

}