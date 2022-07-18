use rand::seq::SliceRandom;
use rand::thread_rng;
pub fn main() -> Result<Vec<u8>, String> {
  let mut numbers = Vec::new();
  for i in 1..=75 {
    numbers.push(i as u8);
  }
  let mut rng = thread_rng();
  numbers.shuffle(&mut rng);
  Ok(numbers)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_test() {
    let ret = match main() {
      Ok(v) => v,
      Err(_) => vec![],
    };

    assert_eq!(ret.len(), 75);
  }
}
