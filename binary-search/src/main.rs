fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
    let (mut min, mut max) = (0, nums.len());
    while min <= max {
        let mid = (min + max) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            min = mid + 1
        } else {
            max = mid - 1
        }
    }
    return -1;
}
fn main() {
    let target: i32 = 1;
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ret = binary_search(&nums, target);
    println!("Bs:nums{:?} ,{} position is {}", nums, target, ret);
}
