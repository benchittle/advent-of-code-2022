pub fn run() {
    println!("Hello, World!");

    let contents = std::fs::read_to_string("input.txt")
        .expect("not here");

    println!("{}", contents);

    let mut grouped_calories = contents.split("\n\n");
    let mut max = 0;

    for group in grouped_calories {
        let mut current = 0;
        for calories in group.split("\n") {
            current += calories.parse::<i32>().unwrap();
        }
        
    }

    println!("Max calories is {}", max);
}