use raoc2024::run;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Lines;

// rules は， n: [prohibit_after]
// の順で持ってて，それはその数はそれ以降にこの数を禁止するというルール
fn load_rules(ls: &mut Lines) -> HashMap<u32, HashSet<u32>> {
    let mut rules = HashMap::new();
    for l in ls {
        if l.is_empty() {
            // 区切れ目に達した
            return rules;
        }
        let ns: Vec<u32> = l.split("|").map(|n| n.parse().unwrap()).collect();
        rules
            .entry(ns[1])
            .and_modify(|e| {
                e.insert(ns[0]);
            })
            .or_insert(HashSet::from([ns[0]]));
    }
    rules
}

fn is_valid(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let mut prohibited: HashSet<u32> = HashSet::new();
    for page in update {
        if prohibited.contains(page) {
            return false;
        }
        if let Some(r) = rules.get(page) {
            // TODO : is this good?
            prohibited = prohibited.union(r).copied().collect();
        }
    }
    true
}
fn get_middle_page(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn part1(s: &str) {
    let mut lines = s.lines();
    let rules = load_rules(&mut lines);
    let updates: Vec<Vec<u32>> = lines
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    let ans: u32 = updates
        .iter()
        .filter(|pages| is_valid(&rules, pages))
        .map(|ps| get_middle_page(ps))
        .sum();
    println!("{ans}");
}

fn part2(s: &str) {
    let mut lines = s.lines();
    let rules = load_rules(&mut lines);
    let sources: Vec<Vec<u32>> = lines
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    // .filter(|pages| !is_valid(&rules, pages))
}

fn main() {
    run(&[part1, part2]);
}
