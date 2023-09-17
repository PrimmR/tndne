use rand::prelude::*;

use crate::ini::Config;

fn generate() -> String {
    let mut rng = rand::thread_rng();

    format!("{:0>3}", rng.gen_range(0..1000))
}

pub fn display(conf: &Config) -> String {
    let pref = conf.location.to_prefix();
    let suf = if !conf.evil {
        generate()
    } else {
        String::from("666")
    };

    format!("{pref}{suf}")
}
