// MEMO: shuffle
use rand::seq::SliceRandom;
use rand::thread_rng;
// MEMO: gen_range
use rand::Rng;
use std::cmp;
use std::iter;

// MEMO: ユニークなランダム配列を作る際に利用（ここでは利用していない）
fn _generate_unique_random_numbers(max_num: usize) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for i in 1..=max_num {
        nums.push(i as u32);
    }
    let mut rng = thread_rng();
    nums.shuffle(&mut rng);
    nums
}

fn generate_pascals_random_numbers_triangle(depth: usize, max_num: usize) -> Vec<Vec<u32>> {
    let mut data: Vec<Vec<u32>> = Vec::new();
    let mut rng = thread_rng();
    for i in 0..depth {
        let mut data2 = vec![];
        for _ in 0..i + 1 {
            data2.push(rng.gen_range(1..max_num as u32));
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

// MEMO: pascals-triangleと同じ
fn print_pascals_triangle(data: &Vec<Vec<u32>>) {
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

fn sum_min_path(data: Vec<Vec<u32>>) -> u32 {
    let mut tree_sum = data.clone();
    let mut position: usize = 1;
    let triangle_length = data.len();
    while position < triangle_length {
        let line = data[position].clone();
        let mut line_path_sum: Vec<u32> = Vec::new();
        for (index, _) in line.iter().enumerate() {
            let sum_value: u32;
            if index == 0 {
                sum_value = line[index] + tree_sum[position - 1][0];
            } else if index == line.len() - 1 {
                sum_value = line[index] + tree_sum[position - 1][index - 1];
            } else {
                let min_path = cmp::min(
                    tree_sum[position - 1][index - 1],
                    tree_sum[position - 1][index],
                );
                sum_value = line[index] + min_path;
            }
            line_path_sum.push(sum_value);
        }
        tree_sum[position] = line_path_sum;

        position += 1;
    }
    match tree_sum[triangle_length - 1].iter().min() {
        Some(n) => *n,
        None => unreachable!(),
    }
}
fn main() {
    let data = generate_pascals_random_numbers_triangle(4, 30);
    // println!("{:?}", generate_unique_random_numbers(50));
    print_pascals_triangle(&data);
    println!("min => {}", sum_min_path(data));
}
