const NUMBERS_MAX_LENGTH: usize = 25;
const NUMBERS_CENTER_POSITION: usize = 12;

pub fn bingo(numbers: Vec<u8>) -> Result<Vec<Vec<u8>>, String> {
  let mut resutl_numbers: Vec<Vec<u8>> = Vec::new();
  // TODO: エラーハンドリング
  if numbers.len() < NUMBERS_MAX_LENGTH {
    return Err(String::from("error"));
  }
  for y in 0..NUMBERS_MAX_LENGTH {
    resutl_numbers.push(Vec::new());
    for x in 0..NUMBERS_MAX_LENGTH {
      let i = y * NUMBERS_MAX_LENGTH + x;
      if i == NUMBERS_CENTER_POSITION {
        resutl_numbers[y].push(0);
      } else {
        resutl_numbers[y].push(numbers[i] as u8);
      }
    }
  }
  Ok(resutl_numbers)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bingo_test() {
    // Err
    let numbers: Vec<u8> = Vec::new();
    assert_eq!(bingo(numbers), Err(String::from("error")));
    // Err
    let numbers: Vec<u8> = vec![
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    assert_eq!(bingo(numbers), Err(String::from("error")));
    // true
    let numbers: Vec<u8> = vec![
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];
    let ans: Vec<Vec<u8>> = vec![
      [1, 2, 3, 4, 5].to_vec(),
      [6, 7, 8, 9, 10].to_vec(),
      [11, 12, 0, 14, 15].to_vec(),
      [16, 17, 18, 19, 20].to_vec(),
      [21, 22, 23, 24, 25].to_vec(),
    ];
    assert_eq!(bingo(numbers), Ok(ans));
  }
}
