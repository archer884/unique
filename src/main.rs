use std::io::{self, Read, Write};

use clap::{Clap, crate_authors, crate_description, crate_version};
use hashbrown::HashSet;

/// A line-by-line text stream filter
///
/// By default, unique reads an entire text stream at once, line by line, printing only unique
/// elements of the stream.
#[derive(Clap, Clone, Debug)]
#[clap(author = crate_authors!(), about = crate_description!(), version = crate_version!())]
struct Opt {
    /// Causes unique to print only non-unique elements. Elements print only once.
    #[clap(short = 'i', long = "invert")]
    invert: bool,
}

fn main() -> io::Result<()> {
    let options = Opt::parse();
    let mut buf = String::new();

    let handle = io::stdout();
    if options.invert {
        inverted_filter(input(&mut buf)?, handle.lock())
    } else {
        filter(input(&mut buf)?, handle.lock())
    }
}

fn filter<'a>(text: impl IntoIterator<Item = &'a str>, mut out: impl Write) -> io::Result<()> {
    let mut set = HashSet::new();
    text.into_iter()
        .filter(|&x| set.insert(x))
        .try_for_each(|x| writeln!(out, "{}", x))
}

fn inverted_filter<'a>(
    text: impl IntoIterator<Item = &'a str>,
    mut out: impl Write,
) -> io::Result<()> {
    let mut set = HashSet::new();
    text.into_iter()
        .filter(|&x| !set.insert(x))
        .try_for_each(|x| writeln!(out, "{}", x))
}

fn input(buf: &mut String) -> io::Result<impl Iterator<Item = &str>> {
    io::stdin().read_to_string(buf)?;
    Ok(buf.lines())
}
