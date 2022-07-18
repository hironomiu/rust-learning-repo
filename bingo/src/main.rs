use std::io;
mod bingo;
mod create_unique_random_numbers;

fn show_bingo(numbers: &Vec<Vec<u8>>) {
    for y in numbers {
        for num in y {
            print!("{:3}", num);
        }
        println!();
    }
}

fn check(numbers: &Vec<Vec<u8>>) -> bool {
    for y in numbers {
        if y[0] == 0 && y[1] == 0 && y[2] == 0 && y[3] == 0 && y[4] == 0 {
            return true;
        }
    }
    for i in 0..5 {
        if numbers[0][i] == 0
            && numbers[1][i] == 0
            && numbers[2][i] == 0
            && numbers[3][i] == 0
            && numbers[4][i] == 0
        {
            return true;
        }
    }
    if numbers[0][0] == 0
        && numbers[1][1] == 0
        && numbers[2][2] == 0
        && numbers[3][3] == 0
        && numbers[4][4] == 0
    {
        return true;
    }
    if numbers[0][4] == 0
        && numbers[1][3] == 0
        && numbers[2][2] == 0
        && numbers[3][1] == 0
        && numbers[4][0] == 0
    {
        return true;
    }
    return false;
}

fn main() {
    let rand_numbers = match create_unique_random_numbers::main() {
        Ok(v) => v,
        Err(_) => panic!("error"),
    };
    let mut numbers = bingo::bingo(rand_numbers).unwrap();
    let mut rand_numbers = match create_unique_random_numbers::main() {
        Ok(v) => v,
        Err(_) => panic!("error"),
    };
    show_bingo(&numbers);
    println!("Start?");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("error");
    loop {
        let num = match rand_numbers.pop() {
            Some(n) => n,
            None => break,
        };

        println!("{}", num);
        let mut flg = false;
        let mut y_position = 0;
        let mut x_position = 0;
        for (y, line) in numbers.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if value == &num {
                    flg = true;
                    y_position = y;
                    x_position = x;
                }
            }
        }
        if flg {
            println!("Hit!");
            numbers[y_position][x_position] = 0;
        }
        show_bingo(&numbers);

        if check(&numbers) {
            println!("!!!!!!BINGO!!!!!!");
            break;
        }

        println!("Next?(push enter key)");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test() {
        // false
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let numbers = bingo::bingo(rand_numbers).unwrap();
        assert_eq!(check(&numbers), false);
        // true
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let mut numbers = bingo::bingo(rand_numbers).unwrap();
        numbers[0][0] = 0;
        numbers[0][1] = 0;
        numbers[0][2] = 0;
        numbers[0][3] = 0;
        numbers[0][4] = 0;
        assert_eq!(check(&numbers), true);
        // true
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let mut numbers = bingo::bingo(rand_numbers).unwrap();
        numbers[1][0] = 0;
        numbers[1][1] = 0;
        numbers[1][2] = 0;
        numbers[1][3] = 0;
        numbers[1][4] = 0;
        assert_eq!(check(&numbers), true);
        // true
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let mut numbers = bingo::bingo(rand_numbers).unwrap();
        numbers[4][0] = 0;
        numbers[4][1] = 0;
        numbers[4][2] = 0;
        numbers[4][3] = 0;
        numbers[4][4] = 0;
        assert_eq!(check(&numbers), true);
        // true
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let mut numbers = bingo::bingo(rand_numbers).unwrap();
        numbers[0][0] = 0;
        numbers[1][1] = 0;
        numbers[2][2] = 0;
        numbers[3][3] = 0;
        numbers[4][4] = 0;
        assert_eq!(check(&numbers), true);
        // true
        let rand_numbers = match create_numbers::main() {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        let mut numbers = bingo::bingo(rand_numbers).unwrap();
        numbers[0][4] = 0;
        numbers[1][3] = 0;
        numbers[2][2] = 0;
        numbers[3][1] = 0;
        numbers[4][0] = 0;
        assert_eq!(check(&numbers), true);
    }
}
