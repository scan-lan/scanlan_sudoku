use super::logic::Puzzle;

pub fn run() {
    let p = Puzzle::new();
    println!("{}", p.solution());
}
