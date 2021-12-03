use rand_core::RngCore;
use rand_pcg::{Lcg64Xsh32, Pcg32};

pub struct Solver<R: RngCore> {
    pub rng: R,
    pub goal: Board,
    pub states: Vec<(isize, Board)>,
    pub closed: std::collections::HashSet<Vec<u8>>,
    pub open_node_count: usize,
    pub open_node_limit: usize,
    pub show_progress: bool,
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
            let distance = board.compute_distance(&self.goal);
            if self.show_progress && self.open_node_count % 1000 == 0 {
                println!(
                    "open_node_count: {:>6}, distance: {:>3}, best score: {}",
                    self.open_node_count, distance, score
                );
            }
            if let Some(result) = board.check_mate(&self.goal, 10) {
                self.result = Some(result);
                return true;
            }
            let mut add_state = |board: Board| {
                if self.closed.contains(&board.cells) {
                    return;
                } else {
                    self.closed.insert(board.cells.clone());
                }
                let score = (distance - board.compute_distance(&self.goal)) * 1000
                    // - board.path.len() as isize
                    + board.index_distance(board.empty_cell, self.goal.empty_cell) * 1000;
                self.states.push((score, board));
            };
            let cs = board.move_candidates();
            if cs[0] {
                let mut board = board.clone();
                board.move_up();
                add_state(board);
            }
            if cs[1] {
                let mut board = board.clone();
                board.move_right();
                add_state(board);
            }
            if cs[2] {
                let mut board = board.clone();
                board.move_down();
                add_state(board);
            }
            if cs[3] {
                let mut board = board.clone();
                board.move_left();
                add_state(board);
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
            states: vec![(0, board)],
            open_node_count: 0,
            open_node_limit: usize::MAX,
            show_progress: false,
            result: None,
        }
    }
}

pub fn print_path(path: &[usize]) {
    print!("len: {} ", path.len());
    for p in path {
        print!("{}, ", p);
    }
    println!();
}

#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub cells: Vec<u8>,
    pub empty_cell: usize,
    pub path: Vec<usize>,
}

impl Board {
    pub fn new(width: usize, cells: Vec<u8>) -> Self {
        let empty_cell = cells.iter().position(|c| *c == 0).unwrap();
        Self {
            width,
            cells,
            empty_cell,
            path: vec![],
        }
    }

    pub fn shuffle(&mut self, n: usize, rng: &mut impl RngCore) {
        for _ in 0..n {
            let cs = self.move_candidates();
            let mut direction = rng.next_u32() as usize % 4;
            while !cs[direction] {
                direction = (direction + 1) % 4;
            }
            match direction {
                0 => self.move_up(),
                1 => self.move_right(),
                2 => self.move_down(),
                3 => self.move_left(),
                _ => unreachable!(),
            }
        }
    }

    pub fn check_mate(&mut self, goal: &Board, cutoff: usize) -> Option<Board> {
        if self.empty_cell == goal.empty_cell {
            return if self.cells == goal.cells {
                Some(self.clone())
            } else {
                None
            };
        }

        if cutoff == 0 {
            return None;
        }

        let cs = self.move_candidates();
        if cs[0] && self.cells[self.empty_cell - self.width] == goal.cells[self.empty_cell] {
            self.move_up();
            let res = self.check_mate(goal, cutoff - 1);
            self.undo();
            if res.is_some() {
                return res;
            }
        }
        if cs[1] && self.cells[self.empty_cell + 1] == goal.cells[self.empty_cell] {
            self.move_right();
            let res = self.check_mate(goal, cutoff - 1);
            self.undo();
            if res.is_some() {
                return res;
            }
        }
        if cs[2] && self.cells[self.empty_cell + self.width] == goal.cells[self.empty_cell] {
            self.move_down();
            let res = self.check_mate(goal, cutoff - 1);
            self.undo();
            if res.is_some() {
                return res;
            }
        }
        if cs[3] && self.cells[self.empty_cell - 1] == goal.cells[self.empty_cell] {
            self.move_left();
            let res = self.check_mate(goal, cutoff - 1);
            self.undo();
            if res.is_some() {
                return res;
            }
        }
        None
    }

    pub fn compute_distance(&self, goal: &Board) -> isize {
        let width = self.width;
        let height = self.cells.len() / self.width;
        let mut distance = 0;
        {
            let mut max_y = height;
            for x in 0..width {
                for y in 0..max_y {
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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
                    let p = x + y * width;
                    if goal.cells[p] != 0 && self.cells[p] == goal.cells[p] {
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

    pub fn print(&self) {
        for y in 0..self.cells.len() / self.width {
            for x in 0..self.width {
                print!("{:>2} ", self.cells[y * self.width + x]);
            }
            println!();
        }
        println!();
    }

    pub fn print_history(mut self) {
        while !self.path.is_empty() {
            self.print();
            self.undo();
        }
    }

    pub fn move_candidates(&self) -> [bool; 4] {
        // up, right, down, left
        let last_cell = self.path.last().cloned();
        let mut candidates = [false; 4];
        if self.empty_cell > self.width - 1 && last_cell != Some(self.empty_cell - self.width) {
            candidates[0] = true;
        }
        if self.empty_cell % self.width != self.width - 1 && last_cell != Some(self.empty_cell + 1)
        {
            candidates[1] = true;
        }
        if self.empty_cell < self.cells.len() - self.width
            && last_cell != Some(self.empty_cell + self.width)
        {
            candidates[2] = true;
        }
        if self.empty_cell % self.width != 0 && last_cell != Some(self.empty_cell - 1) {
            candidates[3] = true;
        }
        candidates
    }

    #[inline]
    pub fn move_up(&mut self) {
        self.path.push(self.empty_cell);
        let next_empty_cell = self.empty_cell - self.width;
        self.cells[self.empty_cell] = self.cells[next_empty_cell];
        self.cells[next_empty_cell] = 0;
        self.empty_cell = next_empty_cell;
    }

    #[inline]
    pub fn move_right(&mut self) {
        self.path.push(self.empty_cell);
        let next_empty_cell = self.empty_cell + 1;
        self.cells[self.empty_cell] = self.cells[next_empty_cell];
        self.cells[next_empty_cell] = 0;
        self.empty_cell = next_empty_cell;
    }

    #[inline]
    pub fn move_down(&mut self) {
        self.path.push(self.empty_cell);
        let next_empty_cell = self.empty_cell + self.width;
        self.cells[self.empty_cell] = self.cells[next_empty_cell];
        self.cells[next_empty_cell] = 0;
        self.empty_cell = next_empty_cell;
    }

    #[inline]
    pub fn move_left(&mut self) {
        self.path.push(self.empty_cell);
        let next_empty_cell = self.empty_cell - 1;
        self.cells[self.empty_cell] = self.cells[next_empty_cell];
        self.cells[next_empty_cell] = 0;
        self.empty_cell = next_empty_cell;
    }

    #[inline]
    pub fn undo(&mut self) {
        let last_cell = self.path.pop().unwrap();
        self.cells[self.empty_cell] = self.cells[last_cell];
        self.cells[last_cell] = 0;
        self.empty_cell = last_cell;
    }

    #[inline]
    pub fn index_distance(&self, left: usize, right: usize) -> isize {
        let (lx, ly) = self.index_to_xy(left);
        let (rx, ry) = self.index_to_xy(right);
        (lx as isize - rx as isize).abs() + (ly as isize - ry as isize).abs()
    }

    #[inline]
    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

#[test]
fn test_distance() {
    let seed = 0;
    let mut rnd = Pcg32::new(seed, 0xa02bdbf7bb3c0a7);

    let initial_board =
        Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    let mut board = initial_board.clone();
    for _ in 0..30 {
        println!("distance: {}", board.compute_distance(&initial_board));
        board.shuffle(1, &mut rnd);
    }
    board.print();
    board.shuffle(100, &mut rnd);
    board.print();
    println!("distance: {}", board.compute_distance(&initial_board));

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
