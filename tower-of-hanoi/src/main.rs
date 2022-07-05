fn print_hanoi(disk_num: u32, starting_point: char, destination: char, support: char) {
    if disk_num < 1 {
        return;
    }
    print_hanoi(disk_num - 1, starting_point, support, destination);
    println!(
        "move {} from {} to {}",
        disk_num, starting_point, destination
    );
    print_hanoi(disk_num - 1, support, destination, starting_point);
}

fn get_hanoi_array(
    disk_num: u32,
    starting_point: char,
    destination: char,
    support: char,
) -> Vec<(u32, char, char)> {
    let mut result: Vec<(u32, char, char)> = Vec::new();

    fn _hanoi(
        disk_num: u32,
        starting_point: char,
        destination: char,
        support: char,
        result: &mut Vec<(u32, char, char)>,
    ) {
        if disk_num < 1 {
            return;
        }
        _hanoi(disk_num - 1, starting_point, support, destination, result);
        result.push((disk_num, starting_point, destination));
        _hanoi(disk_num - 1, support, destination, starting_point, result);
    }
    _hanoi(disk_num, starting_point, support, destination, &mut result);

    return result;
}

fn main() {
    println!("---- START ----");
    print_hanoi(2, 'A', 'B', 'C');
    println!("---- START ----");
    print_hanoi(3, 'A', 'B', 'C');
    println!("---- START ----");
    print_hanoi(4, 'A', 'B', 'C');
    println!("---- START ----");
    println!("{:?}", get_hanoi_array(2, 'A', 'B', 'C'));
    println!("---- START ----");
    println!("{:?}", get_hanoi_array(3, 'A', 'B', 'C'));
    println!("---- START ----");
    println!("{:?}", get_hanoi_array(4, 'A', 'B', 'C'));
}
