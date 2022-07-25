use ulid::Ulid;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4().to_string();
    let ulid = Ulid::new().to_string();
    println!("uuid => {} , ulid => {}", uuid, ulid);
}
