use std::iter;

fn generate_pascals_triangle(depth: usize) -> Vec<Vec<u32>> {
    let mut data: Vec<Vec<u32>> = Vec::new();
    for i in 0..depth {
        let mut data2 = vec![];
        for _ in 0..i + 1 {
            data2.push(1);
        }
        data.push(data2);
    }
    for line in 2..depth {
        for i in 1..line {
            data[line][i] = data[line - 1][i - 1] + data[line - 1][i]
        }
    }
    data
}

fn print_pascals_triangle(data: Vec<Vec<u32>>) {
    let depth: usize = data.len();
    let mut nums: Vec<u32> = vec![];
    for i in 0..depth {
        nums.push(*data[i].iter().max().unwrap());
    }
    let max_num: u32 = *nums.iter().max().unwrap();
    let max_digit: u32 = max_num.to_string().len() as u32;
    let width = max_digit + (max_digit % 2) + 2;
    for (index, line) in data.iter().enumerate() {
        let mut numbers = String::new();
        for j in line {
            let start_blank: String = iter::repeat(String::from(" "))
                .take((width / 2) as usize)
                .collect();
            let end_blank: String = iter::repeat(String::from(" "))
                .take((width / 2 - (j.to_string().len() as u32)) as usize)
                .collect();
            numbers = numbers + &start_blank + &j.to_string() + &end_blank;
        }
        let start_blank: String = iter::repeat(String::from(" "))
            .take(((width / 2) * ((data.len() as u32) - (index as u32))) as usize)
            .collect();
        println!("{}{}", start_blank, numbers);
    }
}

fn main() {
    println!("---- start ----");
    print_pascals_triangle(generate_pascals_triangle(3));
    println!("---- start ----");
    print_pascals_triangle(generate_pascals_triangle(6));
    println!("---- start ----");
    print_pascals_triangle(generate_pascals_triangle(10));
}
