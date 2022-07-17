pub fn bingo(numbers: Vec<u32>) -> Result<Vec<Vec<u32>>, String> {
  let mut resutl_numbers: Vec<Vec<u32>> = Vec::new();
  // TODO: エラーハンドリング
  if numbers.len() < 1 {
    return Err(String::from("error"));
  }
  for y in 0..5 {
    resutl_numbers.push(Vec::new());
    for x in 0..5 {
      let i = y * 5 + x;
      if i == 12 {
        resutl_numbers[y].push(0);
      } else {
        resutl_numbers[y].push(numbers[i] as u32);
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
    let numbers: Vec<u32> = Vec::new();
    assert_eq!(bingo(numbers), Err(String::from("error")));
    // true
    let numbers: Vec<u32> = vec![
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];
    let ans: Vec<Vec<u32>> = vec![
      [1, 2, 3, 4, 5].to_vec(),
      [6, 7, 8, 9, 10].to_vec(),
      [11, 12, 0, 14, 15].to_vec(),
      [16, 17, 18, 19, 20].to_vec(),
      [21, 22, 23, 24, 25].to_vec(),
    ];
    assert_eq!(bingo(numbers), Ok(ans));
  }
}
