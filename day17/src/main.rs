mod cave;
fn main() {
    let mut cave = cave::Cave::new();
    while !cave.is_done() {
        cave.step();
    }
    println!("Part 1: {}", cave.get_highest_point());
}
