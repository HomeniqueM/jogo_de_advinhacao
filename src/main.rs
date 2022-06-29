extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut palpite: String = String::new();
    let _numero_secreto = rand::thread_rng().gen_range(1..101);

    println!("Advinhe o número!");
    println!("Digite o seu palpite.");
loop{
    io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao ler a entrada");

    let palpite: u32 = palpite.trim().parse().expect("Por favor digite um número!");

    println!("Você disse: {}", palpite);
    println!("O número secreto é: {}", _numero_secreto);
    // Comparar o paçpite com o numero secreto
    match palpite.cmp(&_numero_secreto) {
        std::cmp::Ordering::Less => println!("Muito abaixo"),
        std::cmp::Ordering::Greater => println!("Muito alto!"),
        std::cmp::Ordering::Equal => println!("Você acertou!"),
    }
}
}
