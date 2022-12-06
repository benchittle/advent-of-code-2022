
pub fn run() {
    let input = std::fs::read_to_string("input.txt")
        .expect("AHHHHHHHH");

    println!("{}", input);

    let mut score = 0;
    for round in input.split("\n") {
        if let Some((theirs, mine)) = round.split_once(" ") {
            let theirs_char = theirs.chars().next()
                .expect("'theirs' should be single char string");
            let mine_char = mine.chars().next()
                .expect("'mine' should be single char string");

            assert!(theirs_char == 'A' || theirs_char == 'B' || theirs_char == 'C');
            assert!(mine_char == 'X' || mine_char == 'Y' || mine_char == 'Z');

            score += match theirs_char {
                'A' => match mine_char {
                    'X' => Some(4),
                    'Y' => Some(8),
                    'Z' => Some(3),
                    _ => None,
                },
                'B' => match mine_char {
                    'X' => Some(1),
                    'Y' => Some(5),
                    'Z' => Some(9),
                    _ => None
                },
                'C' => match mine_char {
                    'X' => Some(7),
                    'Y' => Some(2),
                    'Z' => Some(6),
                    _ => None
                },
                _ => None,
            }.expect("'theirs_char' must be 'A', 'B', or 'C'. 'mine_char' must be 'X', 'Y', or 'Z'");
            


        } else {
            println!("Unpacking failed");
        }
    }
    println!("Score: {}", score);
}