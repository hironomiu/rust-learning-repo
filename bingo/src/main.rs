use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn create_numbers() -> Vec<u32> {
    let mut numbers = Vec::new();
    for i in 1..=75 {
        numbers.push(i as u32);
    }
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);
    numbers
}

fn bingo(numbers: Vec<u32>) -> Vec<Vec<u32>> {
    let mut resutl_numbers: Vec<Vec<u32>> = Vec::new();
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
    resutl_numbers
}

fn show_bingo(numbers: &Vec<Vec<u32>>) {
    for y in numbers {
        for num in y {
            print!("{:3}", num);
        }
        println!();
    }
}

fn check(numbers: &Vec<Vec<u32>>) -> bool {
    for y in numbers {
        let mut result = 0;
        for i in y.iter().filter(|&x| *x == 0) {
            result += i;
        }
        if result == 5 {
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
    let rand_numbers = create_numbers();
    let mut numbers = bingo(rand_numbers);
    let mut rand_numbers = create_numbers();
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
