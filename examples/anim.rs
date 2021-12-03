use std::io::Read;

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

    solver.show_progress = true;
    solver.search();

    if let Some(result) = solver.result {
        println!("search nodes: {}", solver.open_node_count);
        // println!("score: {}", score);
        result.print();
        println!("path len: {}", result.path.len());
        result.clone().print_history();
        // print_path(&result.path);
    }
}

fn string_to_board(s: &str) -> slidart::Board {
    let mut cells = vec![];
    let width = s.find("\n").unwrap();
    for c in s.chars() {
        match c {
            '#' => cells.push(0),
            '_' => cells.push(1),
            '.' => cells.push(2),
            _ => {}
        }
    }
    slidart::Board::new(width, cells)
}
