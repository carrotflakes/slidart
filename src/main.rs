use rand_pcg::Pcg32;

fn main() {
    let seed = 0;
    let mut rng = Pcg32::new(seed, 0xa02bdbf7bb3c0a7);

    let goal = slidart::Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    let mut board = goal.clone();
    board.shuffle(30, &mut rng);
    slidart::print_path(&board.path);
    println!("distance: {}", slidart::compute_distance(&board, &goal));
    board.path = vec![];
    board.print();
    println!("=== search ===");
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

    // search(board, &initial_board);
}
