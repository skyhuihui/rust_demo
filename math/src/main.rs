use std::panic;

fn main() {
    println!("add!");
    let nums = [2,3,5];
    let total = sum(&nums);
    println!("{:?}", total);
}

fn sum(arrays: &[u32]) -> Option<u32> {
    let result = panic::catch_unwind(|| {
        let mut total = 0;
        for a in arrays.iter() {
            total += a;
        }
        total
    });
    match result {
        Ok(total) => Some(total),
        Err(_) => None,
    }
}