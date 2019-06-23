use proconio::{input, is_stdin_empty};
use rand::Rng;

fn main() {
    loop {
        if is_stdin_empty() {
            break;
        }
        input! {
            from: i32,
            to: i32,
        }

        let rnd = rand::thread_rng().gen_range(from, to);
        println!("random integer from {} to {}: {}", from, to, rnd);
    }
}
