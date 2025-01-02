use std::fs;

const PREV_LEN: usize = 25;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let nums:Vec<u64> = contents.lines().map(|s| s.parse().unwrap()).collect();

    if let Some((index,&invalid_num)) = nums.iter().enumerate().skip(PREV_LEN).find(|&(i,&x)| {
        let start = i - PREV_LEN;
        !nums[start..i].iter().enumerate().any(|(a,&num_a)| {
            nums[start+a+1..i].iter().any(|&num_b| {
                num_a + num_b == x
            })
        })
    }) {
        println!("{}",invalid_num);

        nums[0..index].iter().enumerate().find(|&(i,_)|{
            let mut total = 0;
            let mut smallest = u64::MAX;
            let mut largest = 0;
            for num in nums[i..index].iter() {
                total += num;
                smallest = smallest.min(*num);
                largest = largest.max(*num);
                if total == invalid_num {
                    println!("{} to {} = {}",smallest,largest,smallest+largest);
                    return true;
                }
                if total > invalid_num {
                    break;
                }
            }
            false
        });
    }
}
