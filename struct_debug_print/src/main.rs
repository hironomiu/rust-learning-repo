#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_name(&self) -> &Self {
        println!("{}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("{}", self.age);
        self
    }
}

fn main() {
    println!("デバッグで構造体をprintln!する");
    let p = Person::new("taro", 30);
    println!("{:?}", p);
    println!("{},{}", p.name, p.age);
    println!("おまけ、メソッドチェーン");
    p.say_name().say_age();
    p.say_age().say_name();
}
