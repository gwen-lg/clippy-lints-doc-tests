Demonstration project of how documentation lints are affected by the marker `#[profiling::function]`

```
$ cargo clippy
    Checking clippy-lints-doc-tests v0.1.0 (/home/gwenlg/Projects/rust/clippy-lints-doc-tests)
error: docs for function returning `Result` missing `# Errors` section
 --> src/lib.rs:7:1
  |
7 | pub fn div_hundred_by(value: u32) -> Result<u32, Error> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#missing_errors_doc
  = note: requested on the command line with `-D clippy::missing-errors-doc`

error: docs for function returning `Result` missing `# Errors` section
  --> src/lib.rs:26:1
   |
26 | pub fn div_thousand_by_2(value: u32) -> Result<u32, Error> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#missing_errors_doc

error: could not compile `clippy-lints-doc-tests` (lib) due to 2 previous errors
```
