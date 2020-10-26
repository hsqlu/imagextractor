# imagextractor
A command-line application that extracts a fixed set of metadata from one or more JPEG images specified on the command line, and serializes the data for each to a JSON file.


## Environment

```
active toolchain
----------------

nightly-x86_64-apple-darwin (default)
rustc 1.46.0-nightly (346aec9b0 2020-07-11)
```

## Build & Test

```
cargo build --release

cargo test
```

## Run

```
./target/release/imagextractor images/rotated_CCW90.jpg images/JAM26496.jpg images/JAM19896.jpg

```

## Expected result

There are .json files corresponding to the above input images created under the same folder.

## TODO

    - Improve error handling.
    - Add rustdoc.
    - Enhance the command process. (e.g. Using clap)
    - Add folder recursive process if needed.
    - Performance tuning: Async process for each image/Make it concurrently processing for multi images.