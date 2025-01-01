use std::fs;

fn part1(nums:&Vec<i32>) {
    for (i,a) in nums.iter().enumerate() {
        for b in nums.iter().skip(i+1) {
            if a+b==2020 {
                println!("{} {} {}",a,b,a*b);
            }
        }
    }
}

fn part2(nums:&Vec<i32>) {
    for (i,a) in nums.iter().enumerate() {
        for (k,b) in nums.iter().skip(i+1).enumerate() {
            for c in nums.iter().skip(k+1) {
                if a+b+c==2020 {
                    println!("{} {} {} {}",a,b,c,a*b*c);
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut nums:Vec<i32> = contents.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    nums.sort();

    part1(&nums);
    part2(&nums);
}
