use crate::Board;

pub fn compute_distance1(board: &Board, goal: &Board) -> isize {
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

pub fn compute_distance2(board: &Board, goal: &Board) -> isize {
    let width = board.width;
    let height = board.cells.len() / board.width;
    let mut distance = 0;
    for i in 0..width * height {
        if board.cells[i] == 0 {
            continue;
        }
        let mut d = isize::MAX;
        for j in 0..width * height {
            if board.cells[i] == goal.cells[j] {
                d = d.min(board.index_distance(i, j));
            }
        }
        distance += d * d;// * board.index_distance(i, goal.empty_cell);
    }
    distance
}

pub fn compute_distance3(board: &Board, goal: &Board) -> isize {
    let width = board.width;
    let height = board.cells.len() / board.width;
    let mut used = vec![false; width * height];
    let mut distance = 0;
    for i in 0..width * height {
        if board.cells[i] == 0 {
            continue;
        }
        let mut d = isize::MAX;
        let mut jj = 0;
        for j in 0..width * height {
            if !used[j] && board.cells[i] == goal.cells[j] {
                d = d.min(board.index_distance(i, j));
                jj = j;
            }
        }
        distance += d * d;// * board.index_distance(i, goal.empty_cell);
        used[jj] = true;
    }
    distance
}

#[test]
fn test_distance() {
    let seed = 0;
    let mut rnd = rand_pcg::Pcg32::new(seed, 0xa02bdbf7bb3c0a7);

    let initial_board = Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    let mut board = initial_board.clone();
    for _ in 0..30 {
        println!("distance: {}", compute_distance2(&board, &initial_board));
        board.shuffle(1, &mut rnd);
    }
    board.print();
    board.shuffle(100, &mut rnd);
    board.print();
    println!("distance: {}", compute_distance2(&board, &initial_board));

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
