# bf-again
A little sub-200-line brainfuck interpreter in Rust.

## Usage
It just takes the program in from stdin until it is closed (`^D`).

``` shell
$ cat programs/hello_world.bf | cargo run
   Compiling bf-again v0.1.0 (/home/ckie/git/bf-again)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/bf-again`
Hello World!
```

## License

See `LICENSE` file.
