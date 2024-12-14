use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("could not read the file");

    process(&content);
}

fn process(content: &str) {
    let mut count: i32 = 0;
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let result = process_parts_dampened(&parts);
        if result {
            count += 1;
        }
    }
    eprintln!("count: {}", count);
}

fn process_parts(parts: &Vec<&str>) -> bool {
    let mut result: bool = true;
    let mut old_diff: i32 = 0;
    let mut old_part: Option<i32> = None;
    for part in parts {
        let now = part.parse::<i32>().unwrap();

       let old = match old_part {
            Some(v) => v,
            None => {
                old_part = Some(now);
                continue;
            },
        };

        let new_diff = old - now;

        if (old_diff > 0 && new_diff < 0) || (old_diff < 0 && new_diff > 0)  {
            result = false;
            break;
        }

        if new_diff == 0 || new_diff.abs() > 3 {
            result = false;
            break;
        } 

        old_part = Some(now);
        old_diff = new_diff;
    }

    return result;
}

fn process_parts_dampened(parts: &Vec<&str>) -> bool {
    eprintln!("processing {:?}", parts);
    if process_parts(&parts) {
        println!("full part ok");
        return true;
    }

    for i in 0..parts.len() {
        let mut copy = parts.clone();
        copy.remove(i);
        eprintln!("dampenend {:?}", copy);
        if process_parts(&copy) {
            println!("dampenend ok");
            return true;
        }
    }
    
    return false;
}