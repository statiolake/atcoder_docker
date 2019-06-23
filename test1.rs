use proconio::input;
use rand::Rng;

fn main() {
    input!(n: usize);
    for _ in 0..n {
        input! {
            from: i32,
            to: i32,
        }

        let rnd = rand::thread_rng().gen_range(from, to);
        println!("random integer from {} to {}: {}", from, to, rnd);
    }
}
