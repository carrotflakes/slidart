// Create animation gif
// convert -delay 20 -loop 0 output/*.pgm -sample 400% output.gif

use std::io::{Read, Write};

use slidart::Board;

fn main() {
    let filepath = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("min.pa".to_string());
    let file = std::fs::File::open(filepath).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let l = buf.find("\n\n").unwrap();
    let (header, body) = buf.split_at(l);
    let board = string_to_board(header.trim());
    let goal = string_to_board(body.trim());
    let mut solver = slidart::Solver::new(goal, board);
    let desire_len = 3000;
    // smile: 30
    // rainbow: 56
    // rainbow5: 120
    // four squares: 128
    // move a dot: 68
    // hello: 474
    // carrot2: 1262

    solver.show_progress = true;
    solver.check_mate_cutoff = 10;
    solver.random_walk = 0;
    solver.random_walk_len = 4;
    solver.score_fn = Box::new(|board, distance| -distance * 100 - board.path.len() as isize * 1);
    solver.distance_fn = Box::new(slidart::compute_distance4);
    // solver.distance_fn = Box::new(|a, b| slidart::compute_distance1(a, b) + slidart::compute_distance2(a, b));

    while solver
        .result
        .as_ref()
        .map(|b| b.path.len())
        .unwrap_or(usize::MAX)
        > desire_len
    {
        solver.search();
    }

    if let Some(result) = solver.result {
        println!("search nodes: {}", solver.open_node_count);
        // println!("score: {}", score);
        result.print();
        println!("path len: {}", result.path.len());
        // result.clone().print_history();
        // print_path(&result.path);
        // println!("{}", board_to_pgm(&result));
        output_pgms(&result);
    }
}

fn string_to_board(s: &str) -> Board {
    let mut cells = vec![];
    let width = s.find("\n").unwrap();
    for c in s.chars() {
        match c {
            '#' => cells.push(0),
            '_' => cells.push(1),
            '.' => cells.push(2),
            '0'..='9' => cells.push((c as usize - '0' as usize) as u8),
            _ => {}
        }
    }
    Board::new(width, cells)
}

fn output_pgms(board: &Board) {
    let mut board = board.clone();
    std::fs::create_dir_all("output").unwrap();
    while !board.path.is_empty() {
        std::fs::File::create(format!("output/{:>04}.pgm", board.path.len()))
            .unwrap()
            .write(board_to_pgm(&board).as_bytes())
            .unwrap();
        board.undo();
    }
}

fn board_to_pgm(board: &Board) -> String {
    let height = board.cells.len() / board.width;
    let mut s = format!(
        "P2\n{} {}\n{}\n",
        board.width,
        height,
        board.cells.iter().max().unwrap()
    );
    for y in 0..height {
        for x in 0..board.width {
            let c = board.cells[x + y * board.width];
            s += &format!("{} ", c);
        }
        s.push('\n');
    }
    s
}
