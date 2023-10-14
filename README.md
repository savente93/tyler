# TYLER

Yup, that's the name!

## Why do I exist?

Two words. Adam Savage.
That beautiful human person is the muse, maker, mythbuster extraordinaire who inspired this project.
Adam often makes things that need templates. These huge templates (the projects are big and fabulous) need to be printed on paper.
Paper that is A4 in size and printed upon by home-office based printers.

This requires the template to be tiled. This bit does not have a good software solution.
Not my original thought. Its Adams. And we heard the call. This project aims to answer it!

## How do I run it?

Pump the brakes there champ!
Its still in development. We are making it in Rust for the backend. The other parts are being dreamed up!

#### Developer setup

Not sure how familiar you are with Rust and how to run it.
The following is a gist from a Rust noob (no, really... not being humble) who uses Linux.

- Install rust. Follow [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Clone into this repo. Oh yeah, we're getting basic here.
- Initially, the test images will not be loaded from git-lfs in the `testing_assets` folder
  - to get these install and use `git-lfs`
  - run the following to get the files locally
```bash
git lfs fetch
git lfs install
git lfs checkout
```

That is about it for now.
To run the application from the repo at this stage run the following:


```bash
RUST_BACKTRACE=full cargo run
# or
RUST_BACKTRACE=1 cargo run
```

or if you want to go savage-mode

```bash
cargo build && RUST_BACKTRACE=full target/debug/tyler --help
# and then
cargo build && RUST_BACKTRACE=full target/debug/tyler
```

## Current design assumptions:
- everything is 2D
- all coordinates are positive (i.e. all images live in the (0,0) -> (w,h) quadrent)
- Everying is axis aligned, no paralaxing or rotation is considered.

The above assumptions are to simply development as a starting point, we may relax some of them down the line, but to get off the ground they are necessary.
