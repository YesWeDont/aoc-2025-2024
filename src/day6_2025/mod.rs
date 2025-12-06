use regex::Regex;

type Int = u128;
pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
fn run(part2: bool) {
    let mut operands = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        let first = line.chars().next();
        if let None = first {
            continue;
        }
        if first == Some('*') || first == Some('+') {
            if !part2 {
                let line_data = line
                    .split(' ')
                    .filter_map(|x| {
                        if x.len() == 0 {
                            None
                        } else {
                            Some(x.to_owned())
                        }
                    })
                    .collect::<Vec<_>>();

                println!(
                    "Result: {}",
                    operands
                        .iter()
                        .map(|line: &String| {
                            Box::new(
                                line.split(' ')
                                    .filter_map(|x| {
                                        if x.len() == 0 {
                                            None
                                        } else {
                                            Some(x.to_owned())
                                        }
                                    })
                                    .map(|el| {
                                        el.parse::<Int>()
                                            .map_err(|_| format!("Bad number {el} received"))
                                            .unwrap()
                                    }),
                            ) as Box<dyn Iterator<Item = _>>
                        })
                        .reduce(|acc, el| {
                            Box::new(
                                acc.zip(el)
                                    .zip(line_data.iter())
                                    .map(|((a, b), op)| if op == "+" { a + b } else { a * b }),
                            )
                        })
                        .expect("expected multiple lines of data before operators")
                        .sum::<Int>()
                );
            } else {
                let re = Regex::new(r"([*+])\s+").unwrap();
                println!(
                    "Result: {}",
                    re.captures_iter(&line)
                        .map(|col| {
                            let ops = col
                                .get_match()
                                .range()
                                .rev()
                                .skip(if col.get_match().range().end == line.len() {
                                    0
                                } else {
                                    1
                                })
                                .map(|idx| {
                                    let operand = operands
                                        .iter()
                                        .filter_map(|operands| {
                                            let operands = operands.chars().collect::<Vec<_>>();
                                            if *operands.get(idx)? == ' ' {
                                                None
                                            } else {
                                                operands.get(idx).cloned()
                                            }
                                        })
                                        .collect::<String>();
                                    operand
                                        .parse::<Int>()
                                        .map_err(|_| format!("Received bad integer {operand}"))
                                        .unwrap()
                                });
                            if col.get(1).unwrap().as_str() == "+" {
                                ops.sum::<Int>()
                            } else {
                                ops.product::<Int>()
                            }
                        })
                        .sum::<Int>()
                )
            }
            return;
        } else {
            operands.push(line);
        }
    }
}
