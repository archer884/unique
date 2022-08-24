use std::{
    io::{self, Read, Write},
    process,
};

use clap::Parser;
use hashbrown::HashSet;

/// A line-by-line text stream filter
///
/// By default, unique reads an entire text stream at once, line by line, printing only unique
/// elements of the stream.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
struct Args {
    /// print only non-unique elements
    ///
    /// Only REPEAT elements will be printed. Each element will be printed ONLY ONCE.
    #[clap(short, long)]
    invert: bool,
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let mut buf = String::new();
    if args.invert {
        inverted_filter(input(&mut buf)?, io::stdout().lock())
    } else {
        filter(input(&mut buf)?, io::stdout().lock())
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
