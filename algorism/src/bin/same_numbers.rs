fn main() {
    let input_num = 30;
    let str = same_numbers(input_num);
    println!("same number({}):{}", input_num, str);

    let input_num = 50;
    let str = same_numbers(input_num);
    println!("same number({}):{}", input_num, str);

    let input_num = 30;
    let str = brute_force_search(input_num);
    println!("brute force search({}):{}", input_num, str);

    let input_num = 50;
    let str = brute_force_search(input_num);
    println!("brute force search({}):{}", input_num, str);
}

fn same_numbers(num: i32) -> String {
    let digit = num / 9;
    let num = num % 9;

    let mut ans = String::new();

    for _ in 0..=digit {
        ans += &num.to_string();
    }
    ans
}

fn brute_force_search(num: i32) -> String {
    let mut count = 1;
    let mut i = 1;
    while count <= num {
        let c = check(i);
        if c {
            // println!("i:{},count:{}", i, count);
            if count == num {
                break;
            }
            count += 1;
        }
        i += 1;
    }
    i.to_string()
}

fn check(num: i32) -> bool {
    let num_to_string = num.to_string();
    let mut result = true;
    let mut tmp: char = num_to_string.chars().next().unwrap();
    for ch in num_to_string.chars() {
        if tmp != ch {
            result = false;
            break;
        }
        tmp = ch
    }
    result
}
