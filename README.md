# docker-rust-sandbox
my rust sandbox on Docker

## Usage

```
$ docker build -t rust-sandbox .
$ docker run -it \
-e PROJECT=hello_world \
-v $(pwd)/projects:/projects \
-v $(pwd)/.cargo/registry:/usr/local/cargo/registry \
rust-sandbox
   Compiling hello_world v0.1.0 (file:///projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52 secs
     Running `target/debug/hello_world hello_world`
Hello, world!
```

```
$ docker run -it \
-v $(pwd)/projects:/projects \
-v $(pwd)/.cargo/registry:/usr/local/cargo/registry \
rust-sandbox \
/bin/sh
# ls
hello_world ...
```
