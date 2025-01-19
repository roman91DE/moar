use pager::{Args, Moar};
use std::{env, process::exit};
mod pager;
mod utils;
fn main() {
    let cl_args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let moar_args = if cl_args.len() == 1 {
        Args {
            filepath: Some(cl_args[0].clone()),
        }
    } else {
        println!("USAGE: moar filepath");
        exit(1);
    };

    let mut moar = Moar::new(moar_args).expect("Failed");
    let _ = moar.run();
}
