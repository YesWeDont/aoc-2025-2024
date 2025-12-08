use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::{Rc, Weak},
};

use regex::Regex;

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
    let re = Regex::new(r"(\d+),(\d+),(\d+)$").unwrap();
    let mut points = std::io::stdin()
        .lines()
        .map_while(|line| {
            let line = line.expect("Error reading from stdin");
            if line.len() == 0 {
                None
            } else {
                let capture = re
                    .captures(&line)
                    .ok_or_else(|| format!("Bad line `{line}` does not contain three integers"))
                    .unwrap();
                let num1 = capture.get(1).unwrap().as_str();
                let num1 = num1
                    .parse::<Int>()
                    .map_err(|_| format!("Received bad number `{num1}"))
                    .unwrap();
                let num2 = capture.get(2).unwrap().as_str();
                let num2 = num2
                    .parse::<Int>()
                    .map_err(|_| format!("Received bad number `{num2}"))
                    .unwrap();
                let num3 = capture.get(3).unwrap().as_str();
                let num3 = num3
                    .parse::<Int>()
                    .map_err(|_| format!("Received bad number `{num3}"))
                    .unwrap();
                Some((num1, num2, num3))
            }
        })
        .collect::<Vec<_>>();
    points.first().expect("Points array is empty!");
    let mut pairs = points[0..points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(idx, x)| std::iter::repeat(x).zip(points[idx + 1..points.len()].iter()))
        .collect::<Vec<_>>();
    pairs.sort_by(|(p11, p12), (p21, p22)| {
        (diff_sq(p11.0, p12.0) + diff_sq(p11.1, p12.1) + diff_sq(p11.2, p12.2))
            .cmp(&(diff_sq(p21.0, p22.0) + diff_sq(p21.1, p22.1) + diff_sq(p21.2, p22.2)))
    });
    if part2 {
        // lucky this isn't a doubly-linked tree...
        let mut membership_data = (0..points.len()).collect::<Vec<_>>();
        let mut group_count = points.len();
        let get_leader_of = |idx: usize, membership_data: &[usize]| {
            let mut prev = idx;
            let mut curr = membership_data[idx];
            while prev != curr {
                prev = curr;
                curr = membership_data[curr];
            }
            curr
        };
        fn offset_from<T>(slice: &[T], ptr: *const T) -> usize {
            ((ptr as usize) - (slice.as_ptr() as usize)) / std::mem::size_of::<T>()
        }
        for (pt1, pt2) in pairs {
            let ptr1 = offset_from(&points, pt1);
            let ptr2 = offset_from(&points, pt2);
            if ptr1 >= points.len() {
                panic!("First elem of pair contained invalid reference to point");
            }
            if ptr2 >= points.len() {
                panic!("Second elem of pair contained invalid reference to point");
            }
            let leader1 = get_leader_of(ptr1, &membership_data);
            let leader2 = get_leader_of(ptr2, &membership_data);
            if leader1 != leader2 {
                // this prevents cycles from occuring.
                membership_data[leader1.min(leader2)] = leader1.max(leader2);
                group_count -= 1;
                if group_count == 1 {
                    println!("Result: {}", pt1.0 * pt2.0);
                    break;
                }
            }
        }
    } else {
        let mut conns = HashMap::new();
        for (a, b) in pairs[0..1000].iter() {
            let a = *a;
            let b = *b;
            println!("Connect {:?} {:?}", a, b);
            match conns.get_mut(&a) {
                None => {
                    let mut hs = HashSet::new();
                    hs.insert(b);
                    conns.insert(a, hs);
                }
                Some(a) => {
                    a.insert(b);
                }
            };
            match conns.get_mut(&b) {
                None => {
                    let mut hs = HashSet::new();
                    hs.insert(a);
                    conns.insert(b, hs);
                }
                Some(b) => {
                    b.insert(a);
                }
            };
        }
        let mut sizes = Vec::new();
        while !conns.is_empty() {
            let first = *conns.iter().next().unwrap().0;
            sizes.push(remove_component_including(&mut conns, first));
        }
        sizes.sort_by(|a, b| b.cmp(a));
        println!("{sizes:?}");
        println!(
            "Result: {}",
            sizes
                .into_iter()
                .chain(std::iter::repeat(1))
                .take(3)
                .product::<usize>()
        );
    }
}
fn remove_component_including<T: Clone + std::fmt::Debug + std::hash::Hash + Eq>(
    conns: &mut HashMap<T, HashSet<T>>,
    starting: T,
) -> usize {
    let mut queue = vec![starting];
    let mut size = 0;
    let mut visited = HashSet::new();
    loop {
        let curr = queue.last();
        match curr {
            None => {
                println!("Stack empty, returning {size}");
                return size;
            }
            Some(elem) => {
                println!("Visit node {elem:?}");
                visited.insert(elem.clone());
                let node_conns = conns
                    .get_mut(&elem)
                    .expect("Stumbled on a node that doesn't exist!");
                println!("Get neighbours {node_conns:?}");
                let mut first_call = true;
                match node_conns
                    .extract_if(|x| {
                        if visited.contains(x) {
                            true
                        } else {
                            let called_pre = first_call;
                            first_call = false;
                            called_pre
                        }
                    })
                    .filter(|x| !visited.contains(x))
                    .next()
                {
                    None => {
                        println!("Neighbours empty, decrementing stack...");
                        size += 1;
                        conns.remove(elem);
                        queue.pop();
                    }
                    Some(next) => {
                        println!("Get next target {next:?}");
                        conns
                            .get_mut(&next)
                            .map(|next_conns| next_conns.remove(elem));
                        // at this point, if empty, program will reach it in the next iteration...
                        queue.push(next);
                    }
                }
            }
        }
    }
}
fn diff_sq(a: Int, b: Int) -> Int {
    a * a + b * b - 2 * a * b
}
