use rand::Rng;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input: Vec<i32> = input
            .split(' ')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let (from, to): (i32, i32) = match *input {
            [from, to] => (from, to),
            _ => unreachable!(),
        };

        let rnd = rand::thread_rng().gen_range(from, to);
        println!("random integer from {} to {}: {}", from, to, rnd);
    }
}
