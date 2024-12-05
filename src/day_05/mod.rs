use crate::file_handling;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn first_puzzle() -> u32 {
    let (rules, updates): (Vec<Rule>, Vec<Update>) = parse_input();

    let mut sum_middle = 0;
    for update in updates {
        if is_update_valid(&update, &rules) {
            sum_middle += update[(update.len() - 1) / 2]
        }
    }

    sum_middle
}

pub fn second_puzzle() -> u32 {
    let (rules, updates): (Vec<Rule>, Vec<Update>) = parse_input();

    let mut sum_middle = 0;
    for update in updates {
        if !is_update_valid(&update, &rules) {
            let fixed_update = fix_update(update, &rules);
            sum_middle += fixed_update[(fixed_update.len() - 1) / 2]
        }
    }

    sum_middle
}

fn fix_update(update: Update, rules: &Vec<Rule>) -> Update {
    let mut relevant_rules: HashMap<(u32, u32), &Rule> = HashMap::new();
    for rule in rules {
        let pos_a = update.iter().position(|n| *n == rule.0);
        let pos_b = update.iter().position(|n| *n == rule.1);
        // Both numbers must be present in update
        if pos_a != None && pos_b != None {
            relevant_rules.insert((rule.0, rule.1), rule);
            relevant_rules.insert((rule.1, rule.0), rule);
        }
    }

    let mut fixed_update = update.clone();
    fixed_update.sort_by(|a, b| {
        // Fetch rule relevant to the comparison
        let rule = relevant_rules.get(&(*a, *b));

        // Interpret the rule itself, and switch the order if the numbers came in reversed
        if rule.unwrap().0 > rule.unwrap().1 {
            if *a < *b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            if *a < *b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    });

    fixed_update
}

fn is_update_valid(update: &Update, rules: &Vec<Rule>) -> bool {
    let mut relevant_rules: HashSet<&Rule> = HashSet::new();
    for i in 0..update.len() {
        let num = update[i];
        rules.iter().filter(|r| r.0 == num).for_each(|r| {
            relevant_rules.insert(r);
        });
    }

    for rule in relevant_rules {
        let pos_a = update.iter().position(|n| *n == rule.0);
        let pos_b = update.iter().position(|n| *n == rule.1);
        // Not relevant rule
        if pos_b == None {
            continue;
        }
        if pos_a.unwrap() > pos_b.unwrap() {
            return false;
        }
    }

    true
}

fn parse_input() -> (Vec<Rule>, Vec<Update>) {
    let file_content = file_handling::read_day_input(5);

    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Update> = vec![];
    let mut reading_rules = true;
    for line in file_content.lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let nums: Vec<u32> = line.split("|").map(|s| s.parse::<u32>().unwrap()).collect();
            rules.push((nums[0], nums[1]));
        } else {
            let nums: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            updates.push(nums);
        }
    }

    (rules, updates)
}

type Update = Vec<u32>;
type Rule = (u32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 5248);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 4507);
    }
}
