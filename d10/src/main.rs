use std::fs;

fn calc(nums: &Vec<u64>, pos: usize, end: u64, dp: &mut Vec<u64>) -> u64 {
    if dp[pos] == 0 {
        if nums[pos] == end {
            return 1;
        }
        let mut total = 0;
        for i in pos + 1..nums.len() {
            if nums[i] - nums[pos] <= 3 {
                total += calc(&nums, i, end, dp);
            }
        }
        dp[pos] = total;
    }
    dp[pos]
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut jolts: Vec<u64> = contents
        .lines()
        .map(|s| s.parse().expect("invalid num"))
        .collect();

    let device_jolt = jolts.iter().max().unwrap() + 3 as u64;
    jolts.push(0 as u64);
    jolts.push(device_jolt);
    jolts.sort();

    let diff: Vec<_> = jolts
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let ones = diff.iter().filter(|&x| *x == 1).count();
    let threes = diff.iter().filter(|&x| *x == 3).count();

    println!("{} {} {}", ones, threes, ones * threes);

    let mut dp = vec![0 as u64; jolts.len()];

    println!("{}", calc(&jolts, 0, device_jolt,&mut dp));
}
