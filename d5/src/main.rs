use std::fs;

fn calc(s:&str,start:i32,end:i32)->i32 {
    if s.len()==0 {
        return start;
    }
    let mid = (end - start + 1)/2;
    if s.starts_with("F") || s.starts_with("L") {
        calc(&s[1..],start,start+mid-1)
    } else {
        calc(&s[1..],start+mid,end)
    }
}

fn part1(contents:&str) {
    let max = contents.lines().map(|s| {
        calc(&s[0..7],0,127) * 8 + calc(&s[7..10],0,7)
        }
    ).fold(0,|prev,cur| prev.max(cur));

    println!("{}",max);
}

fn part2(contents:&str) {
    let all:Vec<_> = contents.lines().map(|s| {
        calc(&s[0..7],0,127) * 8 + calc(&s[7..10],0,7)
        }
    ).collect();
    let min = all.iter().min().unwrap();
    let max = all.iter().max().unwrap();

    for i in *min..*max {
        if !all.contains(&i) {
            println!("{}",i);
        }
    }
    
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}
