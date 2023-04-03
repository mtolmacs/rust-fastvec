use std::{mem, sync::RwLock};

use super::fastvec::FastVec;

#[test]
fn can_store_retrieve_drop() {
    static DROPTEST: RwLock<u8> = RwLock::new(0);

    struct Test(usize, &'static str);

    impl Drop for Test {
        fn drop(&mut self) {
            if let Ok(mut write_guard) = DROPTEST.write() {
                *write_guard += 1;
            }
        }
    }

    let mut buf: FastVec<Test, 2> = FastVec::new();
    assert_eq!(buf.cap(), 2);
    assert_eq!(buf.len(), 0);

    buf.push(Test(1, "A"));
    assert_eq!(buf.len(), 1);
    assert!(!buf.is_heap_allocated());

    buf.push(Test(2, "B"));
    assert_eq!(buf.len(), 2);
    assert!(!buf.is_heap_allocated());

    buf.push(Test(3, "C"));
    assert_eq!(buf.len(), 3);
    assert!(buf.is_heap_allocated());
    assert_eq!(buf.cap(), 4);

    let value = buf.pop().unwrap();
    assert_eq!(value.0, 3);
    assert_eq!(value.1, "C");

    mem::drop(buf);
    assert!(DROPTEST.read().unwrap().eq(&2));
}

#[test]
fn large_amount_of_items_push_pop() {
    struct Test(usize, &'static str);

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
    buf.push(Test(14, "N"));
    buf.push(Test(15, "O"));
    buf.push(Test(16, "P"));
    buf.push(Test(17, "Q"));
    buf.push(Test(18, "R"));
    buf.push(Test(19, "S"));
    buf.push(Test(20, "T"));
    buf.push(Test(21, "X"));
    buf.push(Test(22, "Y"));
    buf.push(Test(23, "Z"));
    
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

    assert!(buf.pop().is_none());
    assert!(buf.pop().is_none());
}