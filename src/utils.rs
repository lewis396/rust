#![allow(unused)]

pub mod math {
    type Complex = (f64, f64);
    fn sin(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
    fn cos(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
    fn tan(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
}

pub mod summary {
    struct PathSummary {
        mean: f64,
        stdv: f64,
        skew: f64,
        kurt: f64,
    }
}

//  Stats utils
//  ===========
//  Containins implementations for commonly used statistical functions.
//  
//  Contents
//  --------
//  - mean
//  - standard devaition
//  - skewness
//  - kurtosis
pub mod stat {
    pub fn sample_mean<'a, I>(data: I) -> f64
    where
    I: ExactSizeIterator<Item = &'a f64>,
    {
        let len: f64 = data.len() as f64;
        data.sum::<f64>() as f64 / len
    }

    pub fn vrnc<'a, I>(it: I) -> f64
    where
    I: ExactSizeIterator<Item = &'a f64>,
    {
        let len: f64 = it.len() as f64;
        let s:  &mut f64 = &mut 0.0;
        let ss: &mut f64 = &mut 0.0;
        for &item in it {
            *s += item;
            *ss += item * item;
        }
        (*ss - *s/len) / (len-1.0)
    }

    pub fn sample_variance<'a, I>(it: I, ub: bool) -> f64 
    where 
    I: ExactSizeIterator<Item = &'a f64> {
        let l = it.len() as f64;
        let sums = it.fold((0.0,0.0), |acc, x| (acc.0+x,acc.1+x*x));
        let v = sums.1 - sums.0 * sums.0 / l;
        match ub {
            false => v / l,
            true => v / (l-1.0)
        }
    }

    pub fn sample_stddev<'a, I>(it: I, ub: bool) -> f64 
    where 
    I: ExactSizeIterator<Item = &'a f64> {
        sample_variance(it, true).sqrt()
    }

    pub fn stdv<'a, I>(it: I) -> f64
    where
    I: ExactSizeIterator<Item = &'a f64>,
    {
        vrnc(it).sqrt()
    }

    pub fn skew<'a, I>(data: I) -> f64
    where
    I: ExactSizeIterator<Item = &'a f64>,
    {
        let len: f64 = data.len() as f64;
        data.sum::<f64>() as f64 / len
    }

    pub fn kurt<'a, I>(data: I) -> f64
    where
    I: ExactSizeIterator<Item = &'a f64>,
    {
        let len: f64 = data.len() as f64;
        data.sum::<f64>() as f64 / len
    } 
}