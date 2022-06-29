extern crate rand;

use std::io;
use rand::Rng;


fn main() {
   let mut palpite:String = String::new();
   let _numero_secreto = rand::thread_rng().gen_range(1..101);

   println!("Advinhe o número!");
   println!("Digite o seu palpite.");


   io::stdin().read_line(&mut palpite).expect("Falha ao ler a entrada");

    println!("Você disse: {}", palpite);
    println!("O número secreto é: {}",_numero_secreto);
}
