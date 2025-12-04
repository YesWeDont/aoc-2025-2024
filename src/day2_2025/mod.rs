use std::collections::HashSet;

use regex::Regex;
type Int = u128;
fn run(part2: bool) {
    let mut result: HashSet<Int> = HashSet::new();
    println!("Enter puzzle input:");
    for input in std::io::stdin().lines() {
        let input = input.expect("Failed to read stdin");
        let re = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
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
    }
    println!("Result: {}", result.iter().sum::<Int>());
}

// generators would make this SO much nicer...
fn divisors(num: u32) -> Box<dyn Iterator<Item = u32>> {
    let ctz = num.trailing_zeros();
    if ctz != 0 {
        Box::new(divisors(num >> ctz).flat_map(move |x| (0..=ctz).map(move |pow| x << pow)))
    } else if num == 1 {
        Box::new([1].into_iter())
    } else {
        let sqrt = num.isqrt();
        let closure = move |a| {
            if num % a == 0 {
                Some([a, num / a])
            } else {
                None
            }
        };
        if sqrt * sqrt == num {
            Box::new(
                (1..sqrt)
                    .step_by(2)
                    .filter_map(closure)
                    .flat_map(|a| a)
                    .chain([sqrt]),
            )
        } else {
            Box::new((1..=sqrt).step_by(2).filter_map(closure).flat_map(|a| a))
        }
    }
}
fn do_range(start: Int, end: Int, pow_here: u32, result: &mut HashSet<Int>, part2: bool) {
    let pows_to_check: Box<dyn Iterator<Item = u32>> = if part2 {
        divisors(pow_here + 1)
    } else {
        // day 1: only check 2
        if pow_here % 2 == 0 {
            Box::new([].into_iter())
        } else {
            Box::new([(pow_here + 1) / 2].into_iter())
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
