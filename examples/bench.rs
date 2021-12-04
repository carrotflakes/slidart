use rand_pcg::Pcg32;
use slidart::{Board, Solver};

fn main() {
    let mut attempt = 0;
    let mut succeeded = 0;
    let mut total_path_length = 0;
    let mut rng = Pcg32::new(0, 0xa02bdbf7bb3c0a7);
    let time = std::time::Instant::now();

    for _ in 0..1000 {
        let goal = Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
        let mut board = goal.clone();
        board.shuffle(100, &mut rng);
        let mut solver = Solver::new(goal, board);
        solver.show_progress = false;
        solver.open_node_limit = 10000;
        solver.distance_fn = Box::new(slidart::compute_distance2);

        attempt += 1;
        if solver.search() {
            succeeded += 1;
            total_path_length += solver.result.unwrap().path.len();
        }
    }

    println!("result: {}/{}", succeeded, attempt);
    println!(
        "average path length: {}",
        total_path_length as f64 / succeeded as f64
    );
    println!("time: {:?}", time.elapsed());
}
