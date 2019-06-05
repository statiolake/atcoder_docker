use std::env;
use std::path::Path;

macro_rules! target_features {
    () => {
        unreachable!("normal target should catch anything")
    };

    ($name:tt: $(@second)? [], $($rest:tt)*) => {
        $name
    };

    ($name:tt: [$feature:tt $($features_rest:tt)*], $($rest:tt)*) => {{
        let clo = || {
            if is_x86_feature_detected!($feature) {
                target_features!($name: @second [$($features_rest)*], $($rest)*)
            } else {
                target_features!($($rest)*)
            }
        };
        clo()
    }};

    ($name:tt: @second [, $feature:tt $($features_rest:tt)*], $($rest:tt)*) => {
        if is_x86_feature_detected!($feature) {
            target_features!($name: @second [$($features_rest)*], $($rest)*)
        } else {
            target_features!($($rest)*)
        }
    };
}

fn main() {
    let base_dir = env::args()
        .nth(1)
        .expect("please specify the base directory.");
    let base_dir = Path::new(&base_dir);

    let feature = include!("../../target_features");

    let dir = base_dir.join(feature);
    println!("{}", dir.display());
}
