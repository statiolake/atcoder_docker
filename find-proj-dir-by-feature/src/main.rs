use std::env;
use std::path::Path;

macro_rules! support {
    () => {
        "normal"
    };

    ($name:tt $($rest:tt)*) => {
        if is_x86_feature_detected!($name) {
            $name
        } else {
            support!($($rest)*)
        }
    };
}

fn main() {
    let base_dir = env::args()
        .nth(1)
        .expect("please specify the base directory.");
    let base_dir = Path::new(&base_dir);

    let feature = include!("../../feature_to_support");

    let dir = base_dir.join(feature);
    println!("{}", dir.display());
}
