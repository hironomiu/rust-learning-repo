// add cargo.toml -> rand
use rand::seq::SliceRandom;
fn main() {
    let mut nums: Vec<u32> = Vec::new();
    for i in 1..=30 {
        nums.push(i as u32);
    }
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    println!("{:?}", nums);
}
