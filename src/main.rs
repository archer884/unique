use hashbrown::HashSet;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let handle = io::stdout();

    let mut buf = String::new();
    let mut set = HashSet::new();
    let mut out = handle.lock();

    io::stdin().read_to_string(&mut buf)?;

    for line in buf.lines() {
        let line = line.trim();

        if set.insert(line) {
            writeln!(out, "{}", line)?;
        }
    }

    Ok(())
}
