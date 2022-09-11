fn main() {
    let input_num = 30;
    let str = same_numbers(input_num);
    println!("same number({}):{}", input_num, str);

    let input_num = 50;
    let str = same_numbers(input_num);
    println!("same number({}):{}", input_num, str);
}

fn same_numbers(num: i32) -> String {
    let digit = num / 9;
    let num = num % 9;

    let mut ans = String::new();

    for _ in 0..digit {
        ans += &num.to_string();
    }
    ans
}
