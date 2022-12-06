use std::{collections::HashSet, slice::RChunksExactMut};

pub fn run() {
    let offset_a = u32::from('a') - 1;
    let offset_A = u32::from('A') - 27;

    let input = std::fs::read_to_string("input.txt")
        .expect("AHHHHHHHHH");

    let mut sum = 0;
    for line in input.split('\n') {
        let (left, right) = line.split_at(line.len() / 2);
        let right_char_set:HashSet<char> = HashSet::from_iter(right.chars());
        //println!("Line: {}\n\tleft={}\tright={}", line, left, right);
        //println!("HashSet: {:?}", right_char_set);

        for (left_char) in left.chars() {
            if right_char_set.contains(&left_char) {
                //println!("Item in both: {}", left_char);
                if left_char.is_lowercase() {
                    //println!("Adding {}", u32::from(left_char) - offset_a);
                    sum += u32::from(left_char) - offset_a;
                } else {
                    //println!("Adding {}", u32::from(left_char) - offset_A);
                    sum += u32::from(left_char) - offset_A;
                }
                break;
            }
        }
    }
    println!("Sum is {}", sum);
}