extern crate fastvec;
#[macro_use]
extern crate bma_benchmark;
extern crate smallvec;

use fastvec::FastVec;
use smallvec::SmallVec;

struct Test(usize, &'static str);

fn small_push_pop() {
    #[benchmark_stage(i=50_000_000,name="fastvec")]
    fn fastvec_small_push_pop() {
        let mut buf = FastVec::<Test, 3>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));

        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 3);
        assert_eq!(val.1, "C");
                
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 2);
        assert_eq!(val.1, "B");
        
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 1);
        assert_eq!(val.1, "A");
    }
    
    #[benchmark_stage(i=50_000_000,name="vec")]
    fn vec_small_push_pop() {
        let mut buf = Vec::<Test>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));

        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 3);
        assert_eq!(val.1, "C");
                
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 2);
        assert_eq!(val.1, "B");
        
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 1);
        assert_eq!(val.1, "A");
    }
    
    #[benchmark_stage(i=50_000_000,name="smallvec")]
    fn smallvec_small_push_pop<A: smallvec::Array<Item = Test>>() {
        let mut buf = smallvec::SmallVec::<A>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));

        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 3);
        assert_eq!(val.1, "C");
                
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 2);
        assert_eq!(val.1, "B");
        
        let val = buf.pop();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.0, 1);
        assert_eq!(val.1, "A");
    }

    fastvec_small_push_pop();
    vec_small_push_pop();
    smallvec_small_push_pop::<[Test; 3]>();
}

fn large_push_pop() {
    #[benchmark_stage(i=10_000_000,name="fastvec")]
    fn fastvec_large_push_pop() {
        let mut buf = FastVec::<Test, 3>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));
        buf.push(Test(4, "D"));
        buf.push(Test(5, "E"));
        buf.push(Test(6, "F"));
        buf.push(Test(7, "G"));
        buf.push(Test(8, "H"));
        buf.push(Test(9, "I"));
        buf.push(Test(10, "J"));
        buf.push(Test(11, "K"));
        buf.push(Test(12, "L"));
        buf.push(Test(13, "M"));
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
    }

    #[benchmark_stage(i=10_000_000,name="vec")]
    fn vec_large_push_pop() {
        let mut buf = Vec::<Test>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));
        buf.push(Test(4, "D"));
        buf.push(Test(5, "E"));
        buf.push(Test(6, "F"));
        buf.push(Test(7, "G"));
        buf.push(Test(8, "H"));
        buf.push(Test(9, "I"));
        buf.push(Test(10, "J"));
        buf.push(Test(11, "K"));
        buf.push(Test(12, "L"));
        buf.push(Test(13, "M"));
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
    }

    #[benchmark_stage(i=10_000_000,name="smallvec")]
    fn smallvec_large_push_pop<A: smallvec::Array<Item = Test>>() {
        let mut buf = smallvec::SmallVec::<A>::new();
        buf.push(Test(1, "A"));
        buf.push(Test(2, "B"));
        buf.push(Test(3, "C"));
        buf.push(Test(4, "D"));
        buf.push(Test(5, "E"));
        buf.push(Test(6, "F"));
        buf.push(Test(7, "G"));
        buf.push(Test(8, "H"));
        buf.push(Test(9, "I"));
        buf.push(Test(10, "J"));
        buf.push(Test(11, "K"));
        buf.push(Test(12, "L"));
        buf.push(Test(13, "M"));
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some());
        assert!(buf.pop().is_some()); 
    }

    fastvec_large_push_pop();
    vec_large_push_pop();
    smallvec_large_push_pop::<[Test; 3]>();
}


fn main() {
    small_push_pop();
    staged_benchmark_print_for!("smallvec");
    
    staged_benchmark_reset!();
    
    large_push_pop();
    staged_benchmark_print_for!("smallvec");
}