fn encrypt(text: &str, shift: u8) -> Result<String, ()> {
    let code_big_a = 'A' as u8;
    let code_big_z = 'Z' as u8;
    let big_range: u8 = code_big_z - code_big_a + 1;

    let code_small_a = 'a' as u8;
    let code_small_z = 'z' as u8;
    let small_range: u8 = code_small_z - code_small_a + 1;

    Ok(text
        .chars()
        .map(|c| {
            if code_big_a <= c as u8 && c as u8 <= code_big_z {
                ((c as u8 - code_big_a + big_range + shift) % big_range + code_big_a) as char
            } else if code_small_a <= c as u8 && c as u8 <= code_small_z {
                ((c as u8 - code_small_a + small_range + shift) % small_range + code_small_a)
                    as char
            } else {
                c
            }
        })
        .collect())
}

fn main() {
    let dec = "Hello World!!AbCdE&XyZ!!";
    let enc = match encrypt(dec, 3) {
        Ok(v) => v,
        Err(_) => panic!("error"),
    };
    println!("{},{}", dec, enc);
}
