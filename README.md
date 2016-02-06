# Rustlab - A sample project looking at calling Rust code from within Matlab

Details on this project can be found at (http://smitec.io/2016/02/04/allowing-matlab-to-talk-to-rust.html)[http://smitec.io/2016/02/04/allowing-matlab-to-talk-to-rust.html]

## Running

Build the project with `cargo build`.

Copy `librustlab.dylib` and `librustlab.a` (platform dependant) from `target/debug` into `src`

Open Matlab and run `test.m` to see both the `mex` based and `loadlibrary` based method of calling the function.
