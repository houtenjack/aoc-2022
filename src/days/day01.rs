use crate::etc::assets;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let s = assets::read_asset("assets/1.txt");

    let mut acc: u32 = 0;
    let mut all_calories: Vec<u32> = Vec::new();
    for line in s.lines() {
        if let Ok(i) = line.parse::<u32>() {
            acc += i;
        } else {
            // line is empty, group is finished
            let v = acc.to_owned();
            acc = 0;
            all_calories.push(v);
        }
    }

    all_calories.sort();
    all_calories.reverse();

    let sol1: u32 = match all_calories.get(0) {
        Some(i) => i.clone(),
        None => 0,
    };
    let top_three: Vec<u32> = all_calories.get(0..3).unwrap().to_owned();
    let sol2 = top_three.iter().sum();
    (Solution::U32(sol1), Solution::U32(sol2))
}
