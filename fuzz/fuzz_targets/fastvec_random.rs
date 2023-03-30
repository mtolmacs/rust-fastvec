#![no_main]
// NOTE: Adapted from smallvec's fuzz

use fastvec::FastVec;

// There's no point growing too much, so try not to grow
// over this size.
const CAP_GROWTH: usize = 256;

macro_rules! next_usize {
    ($b:ident) => {
        $b.next().unwrap_or(0) as usize
    };
}

macro_rules! next_u8 {
    ($b:ident) => {
        $b.next().unwrap_or(0)
    };
}

fn do_test<const N: usize>(data: &[u8]) -> FastVec<u8, N> {
    let mut v = FastVec::<u8, N>::new();

    let mut bytes = data.iter().copied();

    while let Some(op) = bytes.next() {
        match op % 3 {
            0 => {
                v = FastVec::<u8, N>::new();
            }
            1 => {
                if v.len() < CAP_GROWTH {
                    v.push(next_u8!(bytes)).unwrap()
                }
            }
            2 => {
                v.pop();
            }
            _ => panic!("booo"),
        }
    }
    v
}

fn do_test_all(data: &[u8]) {
    do_test::<1>(data);
    do_test::<2>(data);
    do_test::<7>(data);
    do_test::<8>(data);
}


use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    // Remove the panic hook so we can actually catch panic
    // See https://github.com/rust-fuzz/afl.rs/issues/150
    std::panic::set_hook(Box::new(|_| {}));
    do_test_all(data)
});

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                b'\n' => {}
                b' ' => {}
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        // paste the output of `xxd -p <crash_dump>` here and run `cargo test`
        extend_vec_from_hex(
            r#"
            646e21f9f910f90200f9d9f9c7030000def9000010646e2af9f910f90264
            6e21f9f910f90200f9d9f9c7030000def90000106400f9f9d9f9c7030000
            def90000106400f9d9f9e7f1000000d9f9e7f1000000f9
            "#,
            &mut a,
        );
        super::do_test_all(&a);
    }
}