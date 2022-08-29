#![crate_name = "rpathgen"]
#![crate_type = "bin"]

#![allow(unused)]
#![warn(non_camel_case_types)]
#![warn(non_snake_case)]

use rand::prelude::*;
use rand_distr::StandardNormal;

mod path;
pub use path::path::Path;
pub use path::path::Summary;

mod utils;
pub use utils::math;
pub use utils::stat;

mod tests;

fn generate_random_path(len: usize) -> Vec<f64> {
    //  Simulate a simple arithmetic Brownian motion.
    //  "param"
    //      LEN: const usize 
    //      The size of the underlying path (and returned vector).
    //  :return:
    //      arr: Vec<f64>
    //      The path stored in a std vector. 
    //  :notes:
    //      Currently returning "by-value." Invectigate this.
    let mut rng = rand::thread_rng();
    let mut arr = Vec::new();
    arr.resize(len, 0.0);
    for item in arr.iter_mut() {
        *item = rng.gen::<f64>();
    }
    arr
}

fn generate_brownian_path(len: usize) -> Vec<f64> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr = Vec::new();
    arr.resize(len, 0.0);
    for step in arr.iter_mut() {
        *step = rng.sample(StandardNormal);
    }
    arr
}



struct TestResult {
    stat: f64,
    pval: f64 
}

struct ShapitoWilkTest {
    data: Vec<f64>,
    res:  TestResult
}


fn main() {
    let generated_abm_array = generate_brownian_path(50);
    let generated_abm_path: Path = Path::from_vec(generated_abm_array);

    println!("ABM Path");
    generated_abm_path.print_enumerated();
    println!("{}", generated_abm_path.summarize());
    
    //let generated_abm_path_10m = Path::from_vec(generate_brownian_path(10000000));
    //println!("{}", generated_abm_path_10m.summarize());

    let p2 = Path::from_vec(vec![50.0;100]);
    let m2 = 50.0;
    let v2 = 0.0;

    let var = stat::sample_variance(p2.iter(), true);
    println!("sample varaince: {}", var);

    println!("Testing for nornality");
    
    println!("Hello, world!");
}
