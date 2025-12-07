use std::{collections::HashMap, hash::Hash};

pub(crate) fn part1() {
    println!("======Part 1======");
    run(false)
}
pub(crate) fn part2() {
    println!("======Part 2======");
    run(true)
}
fn run(part2: bool) {
    let mut poses = MultiSet::new();
    let mut splits = 0;
    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        if line.len() == 0 {
            if part2 {
                println!("Result: {}", poses.sum());
            } else {
                println!("Result: {splits}");
            }
            break;
        }
        poses = line
            .chars()
            .enumerate()
            .flat_map(|(pos, char)| {
                if char == 'S' {
                    OneTwoNoneIter::one((pos, 1))
                } else if let Some(acc) = poses.get(&pos) {
                    if char == '^' {
                        splits += 1;
                        if pos == 0 {
                            OneTwoNoneIter::one((pos + 1, acc))
                        } else {
                            OneTwoNoneIter::two((pos + 1, acc), (pos - 1, acc))
                        }
                    } else if char == '.' {
                        OneTwoNoneIter::one((pos, acc))
                    } else {
                        OneTwoNoneIter::none()
                    }
                } else {
                    OneTwoNoneIter::none()
                }
            })
            .collect()
    }
}

// an iterator containing one, two or no items
enum OneTwoNoneIter<T> {
    One(Option<T>),
    Two(Option<T>, Option<T>),
}
impl<T> Iterator for OneTwoNoneIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::One(val) => val.take(),
            Self::Two(val1, val2) => {
                if let Some(val2) = val2.take() {
                    let next = val1.replace(val2);
                    *self = Self::One(val1.take());
                    next
                } else {
                    None
                }
            }
        }
    }
}
impl<T> OneTwoNoneIter<T> {
    fn none() -> Self {
        Self::One(None)
    }
    fn one(item: T) -> Self {
        Self::One(Some(item))
    }
    fn two(item1: T, item2: T) -> Self {
        Self::Two(Some(item1), Some(item2))
    }
}

// a counting hashset
struct MultiSet<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash> FromIterator<(T, usize)> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = (T, usize)>>(iter: I) -> Self {
        let mut map: HashMap<T, usize> = HashMap::new();
        for (key, acc) in iter {
            let new = map.get(&key).map(|f| *f + acc).unwrap_or(acc);
            map.insert(key, new);
        }
        Self(map)
    }
}
impl<T: Eq + Hash> MultiSet<T> {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn get(&self, key: &T) -> Option<usize> {
        self.0.get(key).cloned()
    }
    fn sum(&self) -> usize {
        self.0.iter().map(|(_, v)| v).sum::<usize>()
    }
}
