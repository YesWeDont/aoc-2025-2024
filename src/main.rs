use gumdrop::Options;
mod day1_2025;
mod day2_2025;
mod day3_2025;
mod day4_2025;
mod day5_2025;
mod day6_2025;
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
macro_rules! day_if {
    ($day_num: literal, $day_name: ident, $day_parsed: ident, $part_parsed: ident) => {
        if $day_parsed == $day_num {
            use $day_name::*;
            if $part_parsed == Some(1){
                part1();
            } else if $part_parsed == Some(2){
                part2();
            } else if $part_parsed == None {
                part1();
                part2();
            } else {
                println!(
                    "There are only 2 parts to the 1st day of AoC 2025! Choose a number for [part] from 1 to 2 inclusive or omit it entirely to run all parts."
                );
            }

        }
    };
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
        }
        day_if!(1, day1_2025, day, part);
        day_if!(2, day2_2025, day, part);
        day_if!(3, day3_2025, day, part);
        day_if!(4, day4_2025, day, part);
        day_if!(5, day5_2025, day, part);
        day_if!(6, day6_2025, day, part);
    }
}
