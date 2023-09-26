extern crate fastvec;
#[macro_use]
extern crate bma_benchmark;
extern crate smallvec;

use arrayvec::ArrayVec;
use fastvec::FastVec;
use smallvec::SmallVec;

struct Test(usize, &'static str);

fn small_push_pop() {
    #[benchmark_stage(i = 10_000_000, name = "fastvec")]
    fn fastvec_small_push_pop() {
        let mut buf = FastVec::<Test, 3>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));

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

    #[benchmark_stage(i = 10_000_000, name = "vec")]
    fn vec_small_push_pop() {
        let mut buf = Vec::<Test>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));

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

    #[benchmark_stage(i = 10_000_000, name = "smallvec")]
    fn smallvec_small_push_pop<A: smallvec::Array<Item = Test>>() {
        let mut buf = smallvec::SmallVec::<A>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));

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

    #[benchmark_stage(i = 10_000_000, name = "arrayvec")]
    fn arrayvec_small_push_pop<const Size: usize>() {
        let mut buf = ArrayVec::<Test, Size>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));

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

    arrayvec_small_push_pop::<8>();
    vec_small_push_pop();
    smallvec_small_push_pop::<[Test; 3]>();
    fastvec_small_push_pop();
}

fn large_push_pop() {
    #[benchmark_stage(i = 10_000_000, name = "fastvec")]
    fn fastvec_large_push_pop() {
        let mut buf = FastVec::<Test, 3>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));
        let _ = buf.push(Test(4, "D"));
        let _ = buf.push(Test(5, "E"));
        let _ = buf.push(Test(6, "F"));
        let _ = buf.push(Test(7, "G"));
        let _ = buf.push(Test(8, "H"));
        let _ = buf.push(Test(9, "I"));
        let _ = buf.push(Test(10, "J"));
        let _ = buf.push(Test(11, "K"));
        let _ = buf.push(Test(12, "L"));
        let _ = buf.push(Test(13, "M"));
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

    #[benchmark_stage(i = 10_000_000, name = "vec")]
    fn vec_large_push_pop() {
        let mut buf = Vec::<Test>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));
        let _ = buf.push(Test(4, "D"));
        let _ = buf.push(Test(5, "E"));
        let _ = buf.push(Test(6, "F"));
        let _ = buf.push(Test(7, "G"));
        let _ = buf.push(Test(8, "H"));
        let _ = buf.push(Test(9, "I"));
        let _ = buf.push(Test(10, "J"));
        let _ = buf.push(Test(11, "K"));
        let _ = buf.push(Test(12, "L"));
        let _ = buf.push(Test(13, "M"));
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

    #[benchmark_stage(i = 10_000_000, name = "smallvec")]
    fn smallvec_large_push_pop<A: smallvec::Array<Item = Test>>() {
        let mut buf = smallvec::SmallVec::<A>::new();
        let _ = buf.push(Test(1, "A"));
        let _ = buf.push(Test(2, "B"));
        let _ = buf.push(Test(3, "C"));
        let _ = buf.push(Test(4, "D"));
        let _ = buf.push(Test(5, "E"));
        let _ = buf.push(Test(6, "F"));
        let _ = buf.push(Test(7, "G"));
        let _ = buf.push(Test(8, "H"));
        let _ = buf.push(Test(9, "I"));
        let _ = buf.push(Test(10, "J"));
        let _ = buf.push(Test(11, "K"));
        let _ = buf.push(Test(12, "L"));
        let _ = buf.push(Test(13, "M"));
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

    smallvec_large_push_pop::<[Test; 3]>();
    vec_large_push_pop();
    fastvec_large_push_pop();
}

fn main() {
    small_push_pop();
    staged_benchmark_print_for!("smallvec");

    staged_benchmark_reset!();

    large_push_pop();
    staged_benchmark_print_for!("smallvec");
}
