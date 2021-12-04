mod board;
mod check_mate;
mod distance_fn;
mod solver;

pub use board::*;
pub use check_mate::*;
pub use distance_fn::*;
pub use solver::*;

pub fn print_path(path: &[usize]) {
    print!("len: {} ", path.len());
    for p in path {
        print!("{}, ", p);
    }
    println!();
}
