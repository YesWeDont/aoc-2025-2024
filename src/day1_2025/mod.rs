pub(crate) fn run(part2: bool) {
    println!("Please enter the input data into stdin: ");
    let stdin = std::io::stdin();
    let mut string = String::new();
    let mut state = 50;
    let mut answer = 0;
    let lineno = 0;
    while let Ok(size) = stdin.read_line(&mut string) {
        if size < 2 {
            println!("Result: {}", answer);
            return;
        } else {
            let mut number = string[1..string.len() - 1]
                .parse::<i32>()
                .map_err(|_| format!("Bad integer on line {lineno} ({string})!"))
                .unwrap();
            // get if any the first character of the line (either L or R)
            let char0 = string
                .chars()
                .nth(0)
                .ok_or_else(|| format!("Failed to get first character of line {lineno}({string})!"))
                .unwrap();
            if char0 == 'L' {
                number = -number;
            } else if char0 != 'R' {
                Err(format!("Bad direction on line {lineno} ({string})!")).unwrap()
            }
            state += number;
            if part2 {
                if state > 0 {
                    // For every 100 in the state, we must have crossed once.
                    answer += state / 100
                } else {
                    // same for negative number: for every -100 in the state, we must have crossed
                    // once, but we need to also account for an extra time that we crossed the
                    // boundary going from +ve to negative, unless we started at zero.
                    answer -= state / 100 - if state - number != 0 { 1 } else { 0 };
                }
                // reset the state to a positive number
                state = ((state % 100) + 100) % 100;
            } else {
                if (state % 100) == 0 {
                    answer += 1
                }
            }
            string.clear();
        }
    }
}
pub(crate) fn part1() {
    println!("======Part 1======");
    run(false);
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true);
}

