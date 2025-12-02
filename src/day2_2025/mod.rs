use std::collections::HashSet;

use regex::Regex;
type Int = u128;
fn run(part2: bool) {
    let mut input = String::new();
    println!("Enter puzzle input:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let re = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
    let mut result: HashSet<Int> = HashSet::new();
    for raw_range in re.captures_iter(&input) {
        println!("Recieved raw range {}", raw_range.get_match().as_str());
        let mut start = raw_range
            .get(1)
            .expect("Internal regex error - expected group but none found")
            .as_str()
            .parse::<Int>()
            .map_err(|_| {
                format!(
                    "Bad starting value {} for range {}",
                    raw_range.get(1).unwrap().as_str(),
                    raw_range.get_match().as_str()
                )
            })
            .unwrap();
        let end = raw_range
            .get(2)
            .expect("Internal regex error - expected group but none found")
            .as_str()
            .parse::<Int>()
            .map_err(|_| {
                format!(
                    "Bad ending value {} for range {}",
                    raw_range.get(2).unwrap().as_str(),
                    raw_range.get_match().as_str()
                )
            })
            .unwrap();
        let mut start_pow = start.checked_ilog10().unwrap_or(0);
        let end_pow = end.checked_ilog10().unwrap_or(0);
        if start_pow == end_pow {
            do_range(start, end, start_pow, &mut result, part2);
        } else {
            loop {
                do_range(
                    start,
                    (10 as Int).pow(start_pow + 1).min(end),
                    start_pow,
                    &mut result,
                    part2,
                );
                start_pow += 1;
                start = (10 as Int).pow(start_pow);
                if start >= end {
                    break;
                }
            }
        }
    }
    println!("Result: {}", result.iter().sum::<Int>());
}
fn do_range(start: Int, end: Int, pow_here: u32, result: &mut HashSet<Int>, part2: bool) {
    let pows_to_check = if part2 {
        let mut vec = divisors::get_divisors(pow_here + 1);
        vec.push(1); // doesn't return 1; we need to fix that...
        vec
    } else {
        // day 1: only check 2
        if pow_here % 2 == 0 {
            vec![]
        } else {
            vec![(pow_here + 1) / 2]
        }
    };
    for to_check in pows_to_check {
        if to_check == pow_here + 1 {
            continue;
        }
        let mut start = start;
        let divisor = (0..=pow_here)
            .step_by(to_check as usize) // multiples of to_check
            .map(|pow| (10 as Int).pow(pow))
            // set every to_check-th digit to 1 by summing 10^(n*to_check)...
            .sum::<Int>();

        // (for part 1) an invalid 2 digit number will
        // always be divisible by 11; 4 digits by 101, and so on...
        // and this pattern extends to part 2 except with different 'all-1' patterns
        if start % divisor == 0 {
            result.insert(start);
        }
        loop {
            start = (start + 1).next_multiple_of(divisor);
            if start <= end {
                result.insert(start);
            }
            if start >= end {
                break;
            }
        }
    }
}
pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
