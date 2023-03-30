# FastVec - A SmallVec alternative
In one sentence, the goal is simplicity, composability, modularity and zero dependencies, without compromising on performance or memory usage. This means a lot of unsafe code and it's very early in it's evolution, so use it at your own risk. 

There are some limitations and compromises to achieve these goals:
* Zero-Sized Types are currently not supported
* The stack backing store will take up dead space even when the buffer moves to the heap

The MSRV of this project is Rust version 1.56.1, albeit it should work with older versions as well.

# Goal
The package is intended to provide the basis of other, more complex data structures, so it's scope is limited to being best at only one thing: Being a very fast replacement to basic Vec operations. This also has the added benefit of easier reading and reasoning about the code, making it simpler to maintain, optimize and build. Furthermore build speed is important, so non-essential traits are implemented in separate submodules, so you can include them as needed. Also no macros, which make debugging and reasoning about the code more difficult. The crate also tries to be future-proof and attempts to adhere to strict pointer provenance (albeit it's not enabled in the crate to make it rustc stable compatible).

# Testing and validation
* Rust MIRI validation
* Random usage pattern fuzzing via cargo-fuzz
* Unit test cases

# Contribution
Your contributions are welcome, especially bug reports and testing on various platforms. Feel free to open a PR if you can contribute a fix.

If you would like to contribute an API change, extension or a new trait implementation, please open an issue first and discuss before starting work on a PR (see above re the scope restrictions).

# License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

`SPDX-License-Identifier: Apache-2.0 OR MIT`