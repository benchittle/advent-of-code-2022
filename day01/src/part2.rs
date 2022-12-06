pub fn run() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("AHHHHHHHHHHHHHH");

    //println!("{}", contents);

    let grouped_calories = contents.split("\n\n");
    let mut totals = Vec::new();

    for group in grouped_calories {
        let mut current = 0;
        for calories in group.split("\n") {
            current += calories.parse::<i32>().unwrap();
        }
        totals.push(current);
    }

    totals.sort();
    totals.reverse();

    //println!("{:?}", totals);

    println!("Max calories is {:?}", &totals[0..3].iter().sum::<i32>());
}