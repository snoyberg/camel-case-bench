# Camel case benchmark

A collection of benchmarks for camel casing a large chunk of data. Rules for
implementations:

* Read input from stdin
* Write camel cased output to stdout
* Camel casing works one line at a time
* Treat anything besides upper and lower case ASCII letters (a-z) as spaces
* Any letter following a space must be upper cased
* Any space character must be stripped (besides newlines, which are retained)
* First character in a line may or may not be upper cased, implementation defined

Running the benchmarks:

* Install the Rust toolchain
* Run `cargo run --release`

Running the haskell-based benchmarks

* `curl http://www.gutenberg.org/files/2600/2600-0.txt -o war-and-peace.txt`
* `./main.hs`
