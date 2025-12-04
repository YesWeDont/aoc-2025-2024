pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
macro_rules! su {
    ($x:expr) => {
        $x.checked_sub(1)
    };
}
macro_rules! ad {
    ($x:expr) => {
        $x.checked_add(1)
    };
}
fn cloned_idx<T: Clone>(target: &Vec<Vec<T>>, x: Option<usize>, y: Option<usize>) -> Option<T> {
    target.get(x?)?.get(y?).cloned()
}
pub(crate) fn run(part2: bool) {
    let mut lines = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        if line.len() == 0 {
            break;
        }
        lines.push(line.chars().collect::<Vec<char>>());
    }
    println!(
        "Result: {}",
        if part2 {
            let mut result = 0;
            loop {
                let curr = process(&mut lines);
                result += curr;
                if curr == 0 {
                    break;
                }
            }
            result
        } else {
            process(&mut lines)
        }
    );
}

fn process(lines: &mut Vec<Vec<char>>) -> usize {
    let mut result = Vec::new();
    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            if lines[i][j] == '@' {
                let mut curr = 0;
                if cloned_idx(&lines, su!(i), su!(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, su!(i), Some(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, su!(i), ad!(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, Some(i), su!(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, Some(i), ad!(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, ad!(i), su!(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, ad!(i), Some(j)) == Some('@') {
                    curr += 1
                }
                if cloned_idx(&lines, ad!(i), ad!(j)) == Some('@') {
                    curr += 1
                }
                if curr < 4 {
                    result.push((i, j));
                }
            }
        }
    }
    for (i, j) in result.iter() {
        lines[*i][*j] = '.'
    }
    result.len()
}
