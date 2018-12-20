#![feature(duration_as_u128)]
use std::process::{Command, Stdio};
use std::io::Write;
use std::time::Instant;

struct Bench {
    name: &'static str,
    compiler: &'static str,
    args: Vec<&'static str>,
}

fn get_benches() -> Vec<Bench> {
    vec!
        [ Bench {
            name: "c-inplace",
            compiler: "g++",
            args: vec!["-O2", "impls/c-inplace.c", "-o", "exes/c-inplace"],
        }
        , Bench {
            name: "haskell-chrisdone",
            compiler: "stack",
            args: vec!["ghc", "--", "-O2", "impls/haskell-chrisdone.hs", "-o", "exes/haskell-chrisdone"],
        }
        , Bench {
            name: "rust-inplace",
            compiler: "rustc",
            args: vec!["-O", "impls/rust-inplace.rs", "-o", "exes/rust-inplace"],
        }
        , Bench {
            name: "rust-iterator",
            compiler: "rustc",
            args: vec!["-O", "impls/rust-iterator.rs", "-o", "exes/rust-iterator"],
        }
        , Bench {
            name: "haskell-bytestring-simple",
            compiler: "stack",
            args: vec!["ghc", "--", "-O2", "impls/haskell-bytestring-simple.hs", "-o", "exes/haskell-bytestring-simple"],
        }
        , Bench {
            name: "haskell-string",
            compiler: "stack",
            args: vec!["ghc", "--", "-O2", "impls/haskell-string.hs", "-o", "exes/haskell-string"],
        }
        ]
}

fn make_payload() -> Vec<u8> {
    const LINES: [&'static [u8]; 3] = [
        b"this is a test\n",
        b"i'm still testing things\n",
        b"foobarbabzbafadjfafal;dfjasl;\n",
    ];
    const COUNT: usize = 100000;
    let mut size = 0;
    for line in LINES.iter() {
        size += line.len();
    }
    size *= COUNT;
    let mut res = Vec::with_capacity(size);
    for _ in 0..COUNT {
        for line in LINES.iter() {
            for byte in line.iter() {
                res.push(*byte);
            }
        }
    }
    res
}

fn main() -> Result<(), std::io::Error> {
    let payload = make_payload();
    for bench in get_benches() {
        let mut cmd = Command::new(bench.compiler);
        for arg in bench.args {
            cmd.arg(arg);
        }
        let output = cmd.output()?;
        if !output.status.success() {
            panic!("{:?}", output);
        }

        let exe = format!("exes/{}", bench.name);

        let mut child = Command::new(exe)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;

        // Give any runtime system a chance to finish spinning up
        std::thread::sleep(std::time::Duration::from_millis(500));

        let start = Instant::now();

        {
            child
                .stdin
                .as_mut()
                .expect("This is impossible, stdin was None")
                .write_all(&payload)?;
        }
        child.wait()?;
        println!("{:>#35}:{:>#12}ns", bench.name, start.elapsed().as_nanos());
    }

    Ok(())
}
