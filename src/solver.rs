use rand_core::RngCore;

use crate::{check_mate, Board};

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
    pub random_walk: usize,
    pub random_walk_len: usize,
    pub score_fn: Box<dyn Fn(&Board, isize) -> isize>,
    pub distance_fn: Box<dyn Fn(&Board, &Board) -> isize>,
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
            let distance = (self.distance_fn)(&board, &self.goal);
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
                let score = (self.score_fn)(&board, (self.distance_fn)(&board, &self.goal) - distance);
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
            for _ in 0..self.random_walk {
                let mut board = board.clone();
                board.shuffle(self.random_walk_len, &mut self.rng);
                add_state(board);
            }
            self.states.sort_unstable_by_key(|s| s.0);
            self.states.truncate(1000);
        }
        false
    }
}

impl Solver<rand_pcg::Lcg64Xsh32> {
    pub fn new(goal: Board, board: Board) -> Self {
        let seed = 0;
        let rng = rand_pcg::Pcg32::new(seed, 0xa02bdbf7bb3c0a7);
        Self {
            rng,
            goal,
            closed: vec![board.cells.clone()].into_iter().collect(),
            states: vec![(0, board)].into(),
            open_node_count: 0,
            open_node_limit: usize::MAX,
            show_progress: false,
            check_mate_cutoff: 50,
            random_walk: 1,
            random_walk_len: 10,
            score_fn: Box::new(|_, distance| -distance),
            distance_fn: Box::new(crate::compute_distance2),
            result: None,
        }
    }
}
