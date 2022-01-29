# pinnable

[![docs.rs]](https://docs.rs/pinnable)
[![crates.io]](https://crates.io/crates/pinnable)
[![github]](https://github.com/steffahn/pinnable)
[![MIT / Apache 2.0 licensed]](#License)

[github]: https://img.shields.io/badge/github-steffahn/pinnable-yellowgreen.svg
[crates.io]: https://img.shields.io/crates/v/pinnable.svg
[MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/pinnable.svg
[docs.rs]: https://docs.rs/pinnable/badge.svg


A wrapper for [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html "std::sync::Mutex")
that supports obtaining `Pin<&mut T>` references to the contained value.
It’s a trade-off though, because it can no longer be locked _without_ being pinned.

## License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
