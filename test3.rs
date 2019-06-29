use proconio::{input, is_stdin_empty};
use proconio_derive::derive_readable;
use rand::Rng;

#[derive_readable]
struct Range {
    from: i32,
    to: i32,
}

fn main() {
    loop {
        if is_stdin_empty() {
            break;
        }
        input! {
            range: Range,
        }

        let rnd = rand::thread_rng().gen_range(range.from, range.to);
        println!(
            "random integer from {} to {}: {}",
            range.from, range.to, rnd
        );
    }
}
