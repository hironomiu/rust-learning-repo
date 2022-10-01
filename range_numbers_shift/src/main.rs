fn range_numbers_shift(start: i32, stop: i32, current: i32, shift: i32) -> Result<i32, String> {
    if current >= start && current <= stop {
        if current + shift > stop {
            Ok(current + shift - stop + start)
        } else if current + shift < start {
            Ok(current + shift + stop)
        } else {
            Ok(current + shift)
        }
    } else {
        Err(String::from("error"))
    }
}
fn main() {
    let result = match range_numbers_shift(1, 6, 3, -2) {
        Ok(n) => n,
        Err(n) => panic!("{}", n),
    };
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::range_numbers_shift;

    #[test]
    fn test_range_numbers_shift() {
        assert_eq!(range_numbers_shift(1, 6, 1, 1), Ok(2));
        assert_eq!(range_numbers_shift(1, 6, 1, -1), Ok(6));
        assert_eq!(range_numbers_shift(1, 6, 7, -1), Err(String::from("error")));
    }
}
