use rand_pcg::Pcg32;
use slidart::{Board, Solver};

#[allow(dead_code)]
enum Level {
    Easy,
    Medium,
    Hard,
}

fn main() {
    let level = Level::Medium;
    let mut attempt = 0;
    let mut succeeded = 0;
    let mut total_path_length = 0;
    let mut rng = Pcg32::new(0, 0xa02bdbf7bb3c0a7);
    let time = std::time::Instant::now();

    let goal = match level {
        Level::Easy => Board::new(4, vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]),
        Level::Medium => Board::new(
            4,
            vec![0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1],
        ),
        Level::Hard => Board::new(
            6,
            vec![
                0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5,
                5, 5, 1, 1, 1, 1, 1, 1,
            ],
        ),
    };

    for _ in 0..1000 {
        let mut board = goal.clone();
        board.shuffle(300, &mut rng);
        let mut solver = Solver::new(goal.clone(), board);
        solver.show_progress = false;
        solver.open_node_limit = 10000;
        solver.distance_fn = Box::new(slidart::compute_distance4);

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
