fn main() {
    let mut stack: Vec<f64> = vec![];
    println!("Input RPN(number + - * /):");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let tokens = s.split_whitespace();
    for token in tokens {
        let t = token.trim();

        match t.parse::<f64>() {
            Ok(v) => {
                stack.push(v);
                continue;
            }
            _ => (),
        }

        let b: f64 = stack.pop().unwrap();
        let a: f64 = stack.pop().unwrap();

        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "/" => stack.push(a / b),
            "*" => stack.push(a * b),
            _ => panic!("error,{}", t),
        }
    }
    println!("{}", stack.pop().unwrap());
}
