mod path_test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::Path;

    #[test]
    fn test_const_path() {
        let p1 = Path::new(100, 0.0);
        assert_eq!(*p1.to_vec(), [0.0;100]);

        let p2 = Path::new(10000, 1.0);
        assert_eq!(*p2.to_vec(), [1.0;10000]);

        let p3 = Path::new(5, -1 as f64);
        assert_eq!(*p3.to_vec(), [-1.0, -1.0, -1.0, -1.0, -1.0])
    }

    #[test]
    fn test_path_from_vec() {
        let v1 = vec![1.0;100 as usize];
        let v2 = vec![1.0;100 as usize];

        let p1 = Path::from_vec(v1);
        assert_eq!(*p1.to_vec(), v2);
    }

    #[test]
    fn test_path_iter() {
        let v = vec![0.0, 1.0, 2.0];
        let v_it = v.iter();
        let p = Path::from_vec(vec![0.0, 1.0, 2.0]);

        assert!(v_it.eq(p.iter()));
    }

    #[test]
    fn test_path_into_iter() {
        let v = vec![0.0, 1.0, 2.0];
        let v_iit = v.into_iter();
        let p = Path::from_vec(vec![0.0, 1.0, 2.0]);

        assert!(v_iit.eq(p.into_iter()));
    }

    #[test]
    fn test_path_from_recursion() {
        let size1 = 0.0;
        let init1 = 0.0;
        let rec1 = |x:f64|->f64 { x+1.0 };

        let p1 = Path::from_recursion(3, 1.0, &rec1);
        assert_eq!(*p1.to_vec(), vec![1.0, 2.0, 3.0]);

        let size2 = 5;
        let init2 = 2.0;
        let rec2 = |x:f64|->f64 { x*2.0 };

        let p2 = Path::from_recursion(size2, init2, &rec2);
        let out2 = vec![2.0, 4.0, 8.0, 16.0, 32.0];
        assert_eq!(*p2.to_vec(), out2);
    }

}

//  Tests for the utils::stat module.
mod stat_test {
    use std::iter;

    use super::*;
    use crate::Path;
    use crate::stat;

    #[test]
    fn test_sample_stats() {
        
        let p1 = Path::from_vec(vec![0.0;100]);
        let m1 = 0.0;
        let v1 = 0.0;

        assert_eq!(stat::sample_mean(p1.iter()), m1);
        assert_eq!(stat::stdv(p1.iter()), v1);

        let p2 = Path::from_vec(vec![50.0;100]);
        let m2 = 50.0;
        let v2 = 0.0;

        assert_eq!(stat::sample_mean(p2.iter()), m2);
        assert_eq!(stat::sample_variance(p2.iter(), true), v2);      // TODO This fails.
        

        let v = vec![-1.0, 1.0];
        let vv = v.repeat(5);
        //let vv: Vec<_> = v.iter().cycle().take(v.len() * 5).collect();
        assert_eq!(vv.len(), 10);

        let p3 = Path::from_vec(vv);
        let m3 = 0.0;
        let v3 = 10.0;

        assert_eq!(stat::sample_mean(p3.iter()), m3);
        //assert_eq!(stat::stdv(p2.iter()), v2);
    }
}

mod sde_test {
    use super::*;
    use crate::path::sde::ArithmeticBrownianMotion;

    /// Testing
    /// # Creation of ArithmeticBrownianMotion
    /// # Retrieval of parameters via `get_parameters`
    /// # Maintainence of generic type T.
    #[test]
    fn test_abm_new() {
        /// Testing f64 parameteres 
        let x1: f64 = 0.0;
        let m1: f64 = 0.0;
        let s1: f64 = 1.0;
        let abm = ArithmeticBrownianMotion::new(x1, m1, s1);
        assert_eq!(abm.get_parameters(), (0.0, 0.0, 1.0));

        /// Testing i32 parameters.
        let x2: i32 = 0;
        let m2: i32 = 0;
        let s2: i32 = 1;
        let abm2 = ArithmeticBrownianMotion::new(x2, m2, s2);
        let res2 = (0 as i32, 0 as i32, 1 as i32);
        assert_eq!(abm2.get_parameters(), res2);
    }

    #[test]
    fn test_abm_step() {
        use crate::path::sde::ArithmeticBrownianMotion;
        let x1: f64 = 0.0;
        let m1: f64 = 0.0;
        let s1: f64 = 1.0;
        let abm1 = ArithmeticBrownianMotion::new(x1, m1, s1);
    }
}