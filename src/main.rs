extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Advinhe o número!");
    let numero_secreto = rand::thread_rng().gen_range(1..101);
    
    loop {
        let mut palpite: String = String::new();
        println!("Digite o seu palpite.");
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Você disse: {}", palpite);
      
        // Comparar o paçpite com o numero secreto
        match palpite.cmp(&numero_secreto) {
            std::cmp::Ordering::Less => println!("Muito abaixo"),
            std::cmp::Ordering::Greater => println!("Muito alto!"),
            std::cmp::Ordering::Equal => {
                println!(">> > Você acertou! < <<");
                break;
            }
        }
    }
}
