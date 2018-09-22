# rest-example-rs

This is the second stage of my experiment with staticly link languages for
easily deployable tools.

# Preparation

For rust development you only need to install rustup.
cargo and the stabe or nightly toolchain can be install with it.

# Result

My discoveries are collected in the commit message of the project.

# Conclusion

## Pros

Rust encourage to play simple and add abstractions only when it needed.
The rustc gives human readable information quite often it worst to see
the compile error and look the solution from there, but the official
documentations are full with examples, which lead us to low learning
curve. However the wording of the errors, like borrowing instead of copy,
may strange at first but after reading The rust book it will be easy.

## Cons

Rust is quite a young language which is quite hype oriented.
It wasn't easy to select a simple https library which is stable
enough and simple sync. Hyper with tokio is quite the standard,
but asnc/future are not needed in simple applications.
