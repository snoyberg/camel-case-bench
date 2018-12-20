use std::io::{Read, Write};

struct CamelCase<I> {
    was_space: bool,
    iter: I,
}

fn camel_case<I>(iter: I) -> CamelCase<I> {
    CamelCase {
        was_space: false,
        iter,
    }
}

impl<I: Iterator<Item=Result<u8, std::io::Error>>> Iterator for CamelCase<I> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        loop {
            let b = match self.iter.next() {
                None => {
                    break None;
                }
                Some(Err(e)) => panic!("{:?}", e),
                Some(Ok(b)) => b,
            };
            if b == b'\n' || (b'A' <= b && b <= b'Z') {
                self.was_space = false;
                break Some(b);
            } else if b'a' <= b && b <= b'z' {
                let b = if self.was_space {
                    b - 32
                } else {
                    b
                };
                self.was_space = false;
                break Some(b);
            } else {
                self.was_space = true;
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut buf: [u8; 65536] = unsafe { std::mem::uninitialized() };
    let mut dest = 0;

    for b in camel_case(stdin.lock().bytes()) {
        buf[dest] = b;
        dest += 1;
        if dest >= buf.len() {
            stdout.write_all(&buf).unwrap();
            dest = 0;
        }
    }

    if dest > 0 {
        stdout.write_all(&buf[0..dest]).unwrap();
    }
    /*
        }

        stdout.write_all(&buf[0..dest]).unwrap();
    }
    */
}
