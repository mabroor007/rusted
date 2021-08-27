pub fn nth(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => nth(n - 1) + nth(n - 2),
    }
}

pub fn upto(n: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = vec![];

    for i in 0..n {
        nums.push(nth(i))
    }

    nums
}
