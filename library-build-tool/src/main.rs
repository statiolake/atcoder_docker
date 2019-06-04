use std::env;
use std::path::Path;

macro_rules! target_features {
    (@to $res:ident @rest ) => {};

    (@to $res:ident @rest $name:literal: [$($features:literal),*], $($rest:tt)*) => {
        $res.push(($name, vec![$($features),*]));
        target_features!(@to $res @rest $($rest)*);
    };

    ($($tt:tt)*) => {{
            let mut res = Vec::new();
            target_features!(@to res @rest $($tt)*);
            res
    }};
}

fn main() {
    let path_to_install = env::args()
        .nth(1)
        .expect("please specify library install path");

    let path_to_install = Path::new(&path_to_install);
    let target_features = include!("../../target_features");

    for (name, features) in target_features {
        println!("{} : {:?}", name, features);
    }
}
