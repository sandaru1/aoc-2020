use std::fs;
use regex::Regex;

fn valid(passport: Vec<(&str, &str)>) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required
        .iter()
        .all(|f| passport.iter().filter(|(name, _)| name == f).count() == 1)
}

fn valid2(passport: Vec<(&str, &str)>) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|f| {
            let item = passport.iter().find(|(name,_)| name == f);
            match item {
                Some((_,val)) => {
                    match f {
                        &"byr" => {
                            let year:i32 = val.parse::<i32>().unwrap();
                            year >= 1920 && year<=2002
                        }
                        &"iyr" => {
                            let year:i32 = val.parse::<i32>().unwrap();
                            year >= 2010 && year<=2020
                        }
                        &"eyr" => {
                            let year:i32 = val.parse::<i32>().unwrap();
                            year >= 2020 && year<=2030
                        }
                        &"hgt" => {
                            if val.ends_with("cm") {
                                let height:i32 = val[0..val.len()-2].parse::<i32>().unwrap();
                                height >=150 && height<=193
                            } else if val.ends_with("in") {
                                let height:i32 = val[0..val.len()-2].parse::<i32>().unwrap();
                                height >=59 && height<=76
                            } else {
                                false
                            }
                        }
                        &"hcl" => {
                            let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
                            re.is_match(val)
                        }
                        &"ecl" => {
                            let colors = ["amb","blu","brn", "gry", "grn", "hzl", "oth"];
                            colors.contains(val)
                        }
                        &"pid" => {
                            let re = Regex::new(r"[0-9]{9}").unwrap();
                            val.len()==9 && re.is_match(val)
                        }
                        _ => true
                    }
                }
                None => false
            }
        }
    )

}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let passports: Vec<Vec<_>> = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .flat_map(|s| s.split(" "))
                .map(|f| f.split_once(":").unwrap())
                .collect()
        })
        .collect();

    let cnt = passports.iter().filter(|p| valid2(p.to_vec())).count();

    println!("{}", cnt);
}