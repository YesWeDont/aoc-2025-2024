type Int = u128;
type IntRange = std::ops::RangeInclusive<Int>;
pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
pub(crate) fn run(part2: bool) {
    let mut ranges = Vec::new();
    let mut result = 0;
    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        if line.len() == 0 {
            break;
        }
        let mut nums = line.split('-');
        ranges.push(
            nums.next()
                .unwrap()
                .parse::<Int>()
                .expect("Received a bad number")
                ..=nums
                    .next()
                    .expect("Received line with only 1 number")
                    .parse()
                    .expect("Received a bad number"),
        );
    }
    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        if line.len() == 0 {
            break;
        }
        if !part2 {
            let num = line.parse::<Int>().expect("Bad ingredient ID received");
            if ranges.iter().find(|range| range.contains(&num)).is_some() {
                result += 1;
            }
        }
    }
    if !part2 {
        println!("Result: {result}")
    } else {
        println!(
            "Result: {}",
            combine_while(ranges)
                .into_iter()
                .map(|x| x.end() + 1 - x.start())
                .sum::<Int>()
        );
    }
}
// keep merging until no more merges are possible
fn combine_while(mut ranges: Vec<IntRange>) -> Vec<IntRange> {
    let mut oldsize = ranges.len();
    loop {
        ranges = combine_ranges(ranges);
        if oldsize == ranges.len() {
            return ranges;
        } else {
            oldsize = ranges.len();
        }
    }
}
fn combine_ranges(old_ranges: Vec<IntRange>) -> Vec<IntRange> {
    let mut new_ranges = Vec::<IntRange>::new();
    'old_range: for old_range in old_ranges {
        for new_range in new_ranges.iter_mut() {
            if new_range.start().max(old_range.start()) <= new_range.end().min(old_range.end()) {
                // new_range and old_range overlap; let's combine them
                *new_range = *new_range.start().min(old_range.start())
                    ..=*new_range.end().max(old_range.end());
                continue 'old_range;
            }
        }
        new_ranges.push(old_range);
    }
    new_ranges
}
