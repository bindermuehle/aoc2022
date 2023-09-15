fn main() {
    let mut weight: u64 = 0;
    let mut highest: [u64; 3] = [0, 0, 0];
    let contents = std::fs::read_to_string("src/input.txt").unwrap();
    contents.lines().for_each(|line| match line.parse::<u64>() {
        Ok(num) => weight += num,
        Err(_) => {
            highest = calculate_highest(weight, highest);
            weight = 0
        }
    });
    highest = calculate_highest(weight, highest);
    println!("Highest weight: {}", highest.iter().sum::<u64>())
}

fn calculate_highest(weight: u64, highest: [u64; 3]) -> [u64; 3] {
    let mut slice = highest.to_vec();
    slice.push(weight);
    slice.sort();
    return [slice[1], slice[2], slice[3]];
}
