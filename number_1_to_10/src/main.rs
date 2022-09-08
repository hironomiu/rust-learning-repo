fn main() {
    let nums1: Vec<i32> = (1..=10).map(|s| s).collect();
    println!("{:?}", nums1);

    let nums2: Vec<i32> = vec![1; 10]
        .iter()
        .enumerate()
        .map(|(i, _)| i as i32 + 1)
        .collect();
    println!("{:?}", nums2);
}
