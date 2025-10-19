fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", number);
}
