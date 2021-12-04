use rand_core::RngCore;

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
        self.move_to(self.empty_cell - self.width);
    }

    #[inline]
    pub fn move_right(&mut self) {
        self.move_to(self.empty_cell + 1);
    }

    #[inline]
    pub fn move_down(&mut self) {
        self.move_to(self.empty_cell + self.width);
    }

    #[inline]
    pub fn move_left(&mut self) {
        self.move_to(self.empty_cell - 1);
    }

    #[inline]
    pub fn move_to(&mut self, next_empty_cell: usize) {
        self.path.push(self.empty_cell);
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
