# prustle (Python-like Standard Library)

This Rust crate is a (very, very!) partial implementation of the
[Python Standard Library](https://docs.python.org/2/library/) using
Rust primitives.

Its main purpose is to allow an easy migration path for Python
programmers, providing them with similar function names and APIs that
they are used to.

Also, by giving Python programmers a clue about how the Python
Standard Library APIs can be implemented in Rust, helpful experience
about the language can further the transition into the native use of
the language, further the transition to Rust programming.

## Examples

* Python's `os.name()` provided as `prustle::os::uname()` (implemented
  using the `uname` crate).
* Python's `os.makedirs()` provided as `prustle::os::makedirs()`
  (implemented using `std::fs::create_dir_all`)
* Python's dynamic cast to `bool` value is implemented using
  `Boolable` trait.

## TODO

* `os.listdir`
* ... many other commonly used APIs.

## Contribution

Any help or assistance in extending this package is welcome.

I don't assume that this will grow into a huge project, but
occasionally I intend to extend this project when I find APIs easy
enough to implement. Not everything maps one-to-one, but it's alright
that it does not.
