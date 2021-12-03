use crate::Board;

pub fn check_mate(board: &mut Board, goal: &Board, cutoff: usize) -> Option<Board> {
    if board.empty_cell == goal.empty_cell {
        return if board.cells == goal.cells {
            Some(board.clone())
        } else {
            None
        };
    }

    if cutoff == 0 {
        return None;
    }

    let cs = board.move_candidates();
    let ps = [
        board.empty_cell.overflowing_sub(board.width).0,
        board.empty_cell + 1,
        board.empty_cell + board.width,
        board.empty_cell.overflowing_sub(1).0,
    ];
    for i in 0..4 {
        if cs[i] && board.cells[ps[i]] == goal.cells[board.empty_cell] {
            board.move_to(ps[i]);
            let res = check_mate(board, goal, cutoff - 1);
            board.undo();
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

pub fn check_mate2(board: &mut Board, goal: &Board) -> Option<Board> {
    if board.empty_cell == goal.empty_cell {
        return if board.cells == goal.cells {
            Some(board.clone())
        } else {
            None
        };
    }

    let cs = board.move_candidates();
    let ps = [
        board.empty_cell.overflowing_sub(board.width).0,
        board.empty_cell + 1,
        board.empty_cell + board.width,
        board.empty_cell.overflowing_sub(1).0,
    ];
    let distance = board.index_distance(board.empty_cell, goal.empty_cell);
    for i in 0..4 {
        if cs[i]
            && board.cells[ps[i]] == goal.cells[board.empty_cell]
            && board.index_distance(ps[i], goal.empty_cell) < distance
        {
            board.move_to(ps[i]);
            let res = check_mate2(board, goal);
            board.undo();
            if res.is_some() {
                return res;
            }
        }
    }
    None
}
