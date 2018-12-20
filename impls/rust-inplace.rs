use std::io::{Read, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut buf: [u8; 65536] = unsafe { std::mem::uninitialized() };

    let mut was_space = false;

    loop {
        let size = stdin.read(&mut buf).unwrap();
        if size == 0 { return };

        let mut dest = 0;
        for src in 0..size {
            let b = buf[src];
            if b == b'\n' || (b'A' <= b && b <= b'Z') {
                buf[dest] = b;
                dest += 1;
                was_space = false;
            } else if b'a' <= b && b <= b'z' {
                buf[dest] = if was_space {
                    b - 32
                } else {
                    b
                };
                dest += 1;
                was_space = false;
            } else {
                was_space = true;
            }
        }

        stdout.write_all(&buf[0..dest]).unwrap();
    }
}
