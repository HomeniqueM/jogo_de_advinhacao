use std::io;
fn main() {
   println!("Advinhe o número!");
   println!("Digite o seu palpite.");

   let mut palpite:String = String::new();

   io::stdin().read_line(&mut palpite).expect("Falha ao ler a entrada");

    println!("Você disse: {}", palpite);
}
