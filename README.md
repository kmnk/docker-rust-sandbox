# docker-rust-sandbox
my rust sandbox on Docker

Japanese online book: https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/

## Usage

### Build
```
$ cd /path/to/docker-rust-sandbox
$ docker build -t rust-sandbox .
```

### Run
```
$ cd /path/to/docker-rust-sandbox
$ docker run -it \
-v $(pwd)/projects:/projects \
-v $(pwd)/.cargo/registry:/usr/local/cargo/registry \
rust-sandbox
root@0123456789ab:/projects# ls
hello_world ...
```

```
$ cd /path/to/docker-rust-sandbox
$ docker run -it \
-v $(pwd)/projects:/projects \
-v $(pwd)/.cargo/registry:/usr/local/cargo/registry \
rust-sandbox \
/bin/sh -c "cd hello_world && cargo run"
   Compiling hello_world v0.1.0 (file:///projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52 secs
     Running `target/debug/hello_world hello_world`
Hello, world!
```
