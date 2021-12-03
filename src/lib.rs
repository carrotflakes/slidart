mod board;
mod check_mate;

use rand_core::RngCore;
use rand_pcg::{Lcg64Xsh32, Pcg32};

use check_mate::*;
pub use board::*;

pub struct Solver<R: RngCore> {
    pub rng: R,
    pub goal: Board,
    pub states: Vec<(isize, Board)>,
    // pub states: std::collections::BinaryHeap<(isize, Board)>,
    pub closed: std::collections::HashSet<Vec<u8>>,
    pub open_node_count: usize,
    pub open_node_limit: usize,
    pub show_progress: bool,
    pub check_mate_cutoff: usize,
    pub result: Option<Board>,
}

impl<R: RngCore> Solver<R> {
    pub fn search(&mut self) -> bool {
        while !self.states.is_empty() {
            if self.open_node_count >= self.open_node_limit {
                return false;
            }
            let (score, mut board) = self.states.pop().unwrap();
            self.open_node_count += 1;
            let distance = compute_distance(&board, &self.goal);
            if self.show_progress && self.open_node_count % 1000 == 0 {
                println!(
                    "open_node_count: {:>6}, distance: {:>3}, best score: {}",
                    self.open_node_count, distance, score
                );
            }
            if let Some(result) = check_mate(&mut board, &self.goal, self.check_mate_cutoff) {
                // if let Some(result) = check_mate2(&mut board, &self.goal) {
                self.result = Some(result);
                return true;
            }
            // if board.cells == self.goal.cells {
            //     self.result = Some(board);
            //     return true;
            // }
            let mut add_state = |board: Board| {
                if self.closed.contains(&board.cells) {
                    return;
                } else {
                    self.closed.insert(board.cells.clone());
                }
                let score = (distance - compute_distance(&board, &self.goal)) * 1000
                    // - board.path.len() as isize
                    // + board.index_distance(board.empty_cell, self.goal.empty_cell) * 100
                    ;
                // let index = self
                //     .states
                //     .binary_search_by_key(&score, |s| s.0)
                //     .unwrap_or_else(|x| x);
                // let state = (score, board);
                // self.states.insert(index, state);
                self.states.push((score, board));
            };
            let cs = board.move_candidates();
            let ps = [
                board.empty_cell.overflowing_sub(board.width).0,
                board.empty_cell + 1,
                board.empty_cell + board.width,
                board.empty_cell.overflowing_sub(1).0,
            ];
            for i in 0..4 {
                if cs[i] {
                    let mut board = board.clone();
                    board.move_to(ps[i]);
                    add_state(board);
                }
            }
            // for i in 0..5 {
            //     let mut board = board.clone();
            //     board.shuffle(4, &mut self.rng);
            //     add_state(board);
            // }
            self.states.sort_unstable_by_key(|s| s.0);
            self.states.truncate(1000);
        }
        false
    }
}

impl Solver<Lcg64Xsh32> {
    pub fn new(goal: Board, board: Board) -> Self {
        let seed = 0;
        let rng = Pcg32::new(seed, 0xa02bdbf7bb3c0a7);
        Self {
            rng,
            goal,
            closed: vec![board.cells.clone()].into_iter().collect(),
            states: vec![(0, board)].into(),
            open_node_count: 0,
            open_node_limit: usize::MAX,
            show_progress: false,
            check_mate_cutoff: 50,
            result: None,
        }
    }
}

pub fn compute_distance(board: &Board, goal: &Board) -> isize {
    let width = board.width;
    let height = board.cells.len() / board.width;
    let mut distance = 0;
    let f = |x: usize, y: usize| {
        let p = x + y * width;
        goal.cells[p] != 0 && board.cells[p] == goal.cells[p]
    };
    {
        let mut max_y = height;
        for x in 0..width {
            for y in 0..max_y {
                if f(x, y) {
                    distance -= 1;
                } else {
                    max_y = y;
                    break;
                }
            }
        }
    }
    {
        let mut min_y = 0;
        for x in 0..width {
            for y in (min_y..height).rev() {
                if f(x, y) {
                    distance -= 1;
                } else {
                    min_y = y + 1;
                    break;
                }
            }
        }
    }
    {
        let mut max_y = height;
        for x in (0..width).rev() {
            for y in 0..max_y {
                if f(x, y) {
                    distance -= 1;
                } else {
                    max_y = y;
                    break;
                }
            }
        }
    }
    {
        let mut min_y = 0;
        for x in (0..width).rev() {
            for y in (min_y..height).rev() {
                if f(x, y) {
                    distance -= 1;
                } else {
                    min_y = y + 1;
                    break;
                }
            }
        }
    }
    {
        let mut max_x = width;
        for y in 0..height {
            for x in 0..max_x {
                if f(x, y) {
                    distance -= 1;
                } else {
                    max_x = x;
                    break;
                }
            }
        }
    }
    {
        let mut min_x = 0;
        for y in 0..height {
            for x in (min_x..width).rev() {
                if f(x, y) {
                    distance -= 1;
                } else {
                    min_x = x + 1;
                    break;
                }
            }
        }
    }
    {
        let mut max_x = width;
        for y in (0..height).rev() {
            for x in 0..max_x {
                if f(x, y) {
                    distance -= 1;
                } else {
                    max_x = x;
                    break;
                }
            }
        }
    }
    {
        let mut min_x = 0;
        for y in (0..height).rev() {
            for x in (min_x..width).rev() {
                if f(x, y) {
                    distance -= 1;
                } else {
                    min_x = x + 1;
                    break;
                }
            }
        }
    }

    distance
}

pub fn print_path(path: &[usize]) {
    print!("len: {} ", path.len());
    for p in path {
        print!("{}, ", p);
    }
    println!();
}

#[test]
fn test_distance() {
    let seed = 0;
    let mut rnd = Pcg32::new(seed, 0xa02bdbf7bb3c0a7);

    let initial_board = Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    let mut board = initial_board.clone();
    for _ in 0..30 {
        println!("distance: {}", compute_distance(&board, &initial_board));
        board.shuffle(1, &mut rnd);
    }
    board.print();
    board.shuffle(100, &mut rnd);
    board.print();
    println!("distance: {}", compute_distance(&board, &initial_board));

    // let initial_board = Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    // let mut board = initial_board.clone();
    // for _ in 0..85 {
    //     board.undo();
    // }
    // board.print();

    // dbg!(&board.path);
    // board.path = vec![];
    // dbg!(board.check_mate(&initial_board, 10));
}
