use std::str::Chars;

pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
type Int = u128;
fn run(part2: bool) {
    let allowed_len = if part2 { 12 } else { 2 };
    println!("Enter input:");
    println!(
        "Result: {}",
        std::io::stdin()
            .lines()
            .map(|line| { line.expect("stdin error - failed to read line") })
            .take_while(|line| line.len() != 0)
            .map(|line| extract_max_joltage(line.chars(), line.len() as Int, allowed_len))
            .sum::<Int>()
    )
}
fn extract_max_joltage<'a>(chars: Chars<'a>, iter_len: Int, allowed_len: Int) -> Int {
    let cloned = chars.clone();
    let ret = if iter_len <= allowed_len {
        chars
            .as_str()
            .parse::<Int>()
            .map_err(|_| format!("Received invalid line containing {}", chars.as_str()))
            .unwrap()
    } else {
        let mut saved_chars = chars.clone();
        let TieBreaker(max_digit, max_idx) = chars
            .take((iter_len - allowed_len + 1) as usize)
            .enumerate()
            .map(|(idx, digit)| TieBreaker(digit, usize::MAX - idx)) // big idx = last
            .max()
            .unwrap();
        let max_idx = usize::MAX - max_idx;
        saved_chars.nth(max_idx);
        let dig = max_digit
            .to_digit(10)
            .ok_or_else(|| format!("Received bad digit {max_digit}"))
            .unwrap() as Int;
        if allowed_len == 1 {
            dig
        } else {
            dig * ((10 as Int).pow(allowed_len as u32 - 1))
                + extract_max_joltage(
                    saved_chars,
                    iter_len - (max_idx as Int) - 1,
                    allowed_len - 1,
                )
        }
    };
    println!(
        "Extract max joltage {} iter_len={iter_len} allowed_len={allowed_len} ret={ret}",
        cloned.as_str()
    );
    return ret;
}

// act as a tiebreaker between equal digits: rank first by the digits, then their indicies
#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct TieBreaker(char, usize);
