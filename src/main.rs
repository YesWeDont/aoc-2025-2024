use gumdrop::Options;
mod day1_2025;
mod day2_2025;
#[derive(Options)]
#[options(
    help = "AoC 2024/2025 binary, written by YesWeDont and licensed under GPLv3. Input will be parsed from stdin."
)]
struct OverallOptions {
    #[options(short = "l", help = "Run the 2024 AoC instead")]
    legacy: bool,
    #[options(free, required, help = "Day for which to execute")]
    day: u8,

    #[options(
        free,
        help = "Subtask of specified day for which to execute (default: execute everything)"
    )]
    part: Option<u8>,

    #[options(help_flag, short = "h", help = "Show this help message")]
    help: bool,
}
fn main() {
    let OverallOptions {
        day,
        legacy,
        part,
        help,
    } = OverallOptions::parse_args_default_or_exit();
    println!(
        "Executing day {} of {}",
        day,
        if legacy { "2024" } else { "2025" }
    );
    if legacy {
        todo!()
    } else {
        if day == 0 || day > 12 {
            println!(
                "There are only 12 days in the 2025 AoC season! Choose a number for <day> from 1 to 12 inclusive."
            );
        } else if day == 1 {
            if part == Some(1) {
                day1_2025::part1();
            } else if part == Some(2) {
                day1_2025::part2();
            } else if part == None {
                day1_2025::part1();
                day1_2025::part2();
            } else {
                println!(
                    "There are only 2 parts to the 1st day of AoC 2025! Choose a number for [part] from 1 to 2 inclusive or omit it entirely to run all parts."
                )
            }
        } else if day == 2 {
            if part == Some(1) {
                day2_2025::part1();
            } else if part == Some(2) {
                day2_2025::part2();
            } else if part == None {
                day2_2025::part1();
                day2_2025::part2();
            }
        } else {
            todo!()
        }
    }
}
