mod cave;

fn main() {
    let mut cave = cave::Cave::new();
    while !cave.is_done() {
        cave.step()
    }
}
