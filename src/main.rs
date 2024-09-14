mod remove_duplicates_80;

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let result = remove_duplicates_80::remove_duplicates(&mut nums);
    println!("去重后的结果: {:?}", result);
}
