mod segtree;

pub(crate) fn part1() {
    println!("======Part 1======");
    run::<false>()
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run::<true>()
}
type Int = u128;
fn run<const PART2: bool>() {
    // let allowed_len = if part2 { 12 } else { 2 };
    println!("Enter input:");
    println!(
        "Result: {}",
        std::io::stdin()
            .lines()
            .map(|line| { line.expect("stdin error - failed to read line") })
            .take_while(|line| line.len() != 0)
            .map(|line| if PART2 {
                extract_max_joltage::<12>(line)
            } else {
                extract_max_joltage::<2>(line)
            })
            .sum::<Int>()
    )
}
// act as a tiebreaker between equal digits: rank first by the digits, then their indicies
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
struct TieBreaker(char, usize);

fn extract_max_joltage<const FLIPS: usize>(chars: String) -> Int {
    if FLIPS == 0 {
        0
    } else if chars.len() < FLIPS {
        chars
            .parse()
            .map_err(|_| format!("Bad line `{chars}` received"))
            .unwrap()
    } else {
        let (len, _) = chars
            .chars()
            .enumerate()
            .last()
            .expect("Received line with bad Unicode");
        let len = len + 1;
        let mut segtree = segtree::Segtree::new(
            len,
            |a: &TieBreaker, b: &TieBreaker| std::cmp::max(a, b).clone(),
            || TieBreaker('\0', 0),
        );
        let mut iter = chars.chars().enumerate();
        segtree.data().fill_with(|| {
            let (a, b) = iter.next().unwrap();
            TieBreaker(b, usize::MAX - a)
        });
        segtree.build();
        let mut prev_idx = 0;
        (0..FLIPS)
            .rev()
            .map(|remaining| {
                let TieBreaker(digit, pos) = segtree.query(prev_idx..(len - remaining));
                prev_idx = (usize::MAX - pos) + 1;
                digit
                    .to_digit(10)
                    .ok_or_else(|| format!("Received bad digit {digit}"))
                    .unwrap() as Int
                    * (10 as Int).pow(remaining as u32)
            })
            .sum()
    }
}
