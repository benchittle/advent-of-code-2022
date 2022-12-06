use std::collections::HashSet;
use itertools::Itertools;

pub fn run() {
    let offset_a = u32::from('a') - 1;
    let offset_A = u32::from('A') - 27;

    let input = std::fs::read_to_string("input.txt")
        .expect("AHHHHHHHHH");

    let mut sum = 0;
    let lines = input.split("\n");
    for (line1, line2, line3) in lines.tuples() {
        let line1_set: HashSet<char> = HashSet::from_iter(line1.chars());
        let line2_set: HashSet<char> = HashSet::from_iter(line2.chars());
        let line3_set: HashSet<char> = HashSet::from_iter(line3.chars());

        // Take the intersection of the 3 sets.
        let common: HashSet<char> = [line2_set, line3_set]
            .iter()
            .fold(line1_set.clone(), 
                |acc, set| acc.intersection(set).cloned().collect());
        
        println!("{:?}", common);
        if common.len() != 1 {
            panic!("'common' should have only one element");
        }
         
        let unique = *common.iter().next().unwrap();
        if unique.is_lowercase() {
            //println!("Adding {}", u32::from(left_char) - offset_a);
            sum += u32::from(unique) - offset_a;
        } else {
            //println!("Adding {}", u32::from(left_char) - offset_A);
            sum += u32::from(unique) - offset_A;
        }
    }
    println!("Sum is {}", sum);
}