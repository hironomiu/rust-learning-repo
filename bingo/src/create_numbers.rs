use rand::seq::SliceRandom;
use rand::thread_rng;
pub fn main() -> Vec<u8> {
  let mut numbers = Vec::new();
  for i in 1..=75 {
    numbers.push(i as u8);
  }
  let mut rng = thread_rng();
  numbers.shuffle(&mut rng);
  numbers
}
