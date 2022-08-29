
pub mod path {
    pub use crate::utils::stat;
    use core::ops::{Deref, DerefMut};

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Path {
        data: Vec<f64>,
        size: usize
    }

    impl Path {
        pub fn new(size_: usize, val_: f64) -> Path {
            Path {
                size: size_,
                data: vec![val_ as f64;size_]
            }
        }

        pub fn from_recursion(size_: usize, init_: f64, func_:  &dyn Fn(f64)->f64) -> Path {
            let v = &mut vec![0.0;size_];
            for i in 0..size_ {
                let curr = match i {
                    0   => init_,
                    1.. => func_(v[i-1]),
                    _   => 0.0,
                }; 
                v[i] = curr;
            }

            // To vec can be used independently, *v cannot.
            Path::from_vec(v.to_vec())
        }

        pub fn from_vec(vec: Vec<f64>) -> Path {
            Path {
                size: vec.len(),
                data: vec
            }
        }

        pub fn to_vec(&self) -> &Vec<f64> {
            &self.data
        }

        pub fn print_elements(&self) {
            for element in self.data.iter() {
                println!("{}", *element);
            }
        }

        pub fn print_enumerated(&self) {
            for (i, element) in self.data.iter().enumerate() {
                println!("v[{}] = {}", i, *element);
            }
        }
        
    }

    //  Owning iterator type Path.
    impl IntoIterator for Path {
        type Item = f64;
        type IntoIter = <Vec<f64> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway
    
        fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
        }
    }
    
    // We deref to slice so that we can reuse the slice impls
    impl Deref for Path {
        type Target = [f64];
    
        fn deref(&self) -> &[f64] {
        &self.data[..]
        }
    }

    impl DerefMut for Path {
        fn deref_mut(&mut self) -> &mut [f64] {
        &mut self.data[..]
        }
    }

    impl Summary for Path {
        fn summarize(&self) -> String {
            format!(
                "generated path\n\t mean: {}\t stdv: {}\t skew: {}\t kurt: {}",
                stat::sample_mean(self.iter()),
                stat::stdv(self.iter()),
                stat::skew(self.iter()),
                stat::kurt(self.iter())
            )
        }
    } 
}

pub mod sde {
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;


    /// Default seed for random number generators.
    const DEFAULT_RANDOM_SEED: u64 = 12345;

    /// TemporalState implements describes the state of a time-evolving system,
    /// 
    /// This is deliberately general for now. It is to be expanded.
    pub trait TemporalState {
        type T;
        fn advance(&self, from: f64, to: f64) -> Self::T;
        fn retrieve(&self) -> Self::T;
    }

    pub struct ArithmeticBrownianMotion<T> {
        x0:  T,
        mu:  T,
        sig: T,
        rng: rand::rngs::StdRng,
    }

    /// Implements the ArithmeticBrownianMotion structure
    /// 
    /// T is required to implement the `Copy` trait for consistent retrieval of the 
    /// underlying parameters.
    impl<T: Copy> ArithmeticBrownianMotion<T> {
        /// Constructs a new ArithmeticBrownianMotion
        /// 
        /// Parameters
        /// # x0_:    The initial value.
        /// # mu_:    The constant drift parameter.
        /// # sig_:   The constant volatility parameter.
        ///
        /// Notes
        /// # Non-const due to call to non-const function seed_from_u64.
        pub fn new(x0_: T, mu_: T, sig_: T) -> ArithmeticBrownianMotion<T> {
            ArithmeticBrownianMotion {
                x0: x0_,
                mu: mu_,
                sig: sig_,
                rng: rand::rngs::StdRng::seed_from_u64(DEFAULT_RANDOM_SEED),
            }
        }

        /// Retrieve all parameters of the arithmetic Brownian motion as a tuple.
        ///
        /// Returned order is (INITIAL_VALUE, DRIFT, DIFFUSION)
        pub fn get_parameters(&self) -> (T,T,T) {
            (self.x0, self.mu, self.sig)
        }
    }

    /*pub fn advance_sde()

    pub trait Drift {
        fn drift(&self, t: f64, x: f64) -> f64;
    }

    pub trait Diffusion {
        fn diffusion(&self, t: f64, x: f64) -> f64;
    }

    

    impl Drift for ArithmeticBrownianMotion {
        fn drift(&self, t: f64, x: f64) -> f64 {
            x0 + self.mu * t + self.sig * 
        }
    }*/
}

/* s
pub trait Advance {
    fn advance() -> f64;
}

struct Abm {
    init: f64,
    dynamics: dyn Fn(f64,f64) -> f64,
}

impl<'a> Advance for &'a Abm
{
    type Output = f64;

}*/