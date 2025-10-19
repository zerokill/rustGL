use rand::Rng;
use colored::Colorize;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let number: u32 = rng.gen_range(1..=100);
        println!("Random number: {}", number);
    }

    let float: f32 = rng.gen();
    println!("Random number: {}", float);

    println!("{}", "This is colored!".red().bold());
}
