use std::{
    fs,
    hash::Hash,
    io::{self, Read, Write},
    path::Path,
    process,
};

use clap::Parser;
use either::Either;
use hashbrown::HashSet;
use tempfile::Builder;
use unicase::UniCase;

/// unique text filter
#[derive(Clone, Debug, Parser)]
#[command(about, version)]
struct Args {
    /// an optional input path
    path: Option<String>,

    /// an optional output path
    out: Option<String>,

    /// overwrite input path
    ///
    /// Ignores output path.
    #[arg(short, long)]
    overwrite: bool,

    /// sort output
    #[arg(short, long)]
    sort: bool,

    /// invert output
    ///
    /// Prints only repeated elements.
    #[arg(short, long)]
    invert: bool,

    /// allow blanks
    ///
    /// Ignores blank lines
    #[arg(short = 'b', long)]
    allow_blanks: bool,
}

impl Args {
    fn output_path(&self) -> Option<&str> {
        self.path
            .as_deref()
            .filter(|_| self.overwrite)
            .or(self.out.as_deref())
    }
}

trait Predicate<T: Testable> {
    fn filter(&mut self, s: T) -> bool;
}

trait Testable: Hash + Eq {
    fn non_interesting(&self) -> bool;
}

impl Testable for &str {
    fn non_interesting(&self) -> bool {
        self.trim() == ""
    }
}

#[derive(Debug)]
struct UniquePredicate<T> {
    set: HashSet<T>,
    allow_non_interesting: bool,
}

impl<T> UniquePredicate<T> {
    fn new(allow_non_interesting: bool) -> Self {
        Self {
            set: Default::default(),
            allow_non_interesting,
        }
    }
}

impl<T: Testable> Predicate<T> for UniquePredicate<T> {
    #[inline]
    fn filter(&mut self, value: T) -> bool {
        self.allow_non_interesting && value.non_interesting() || self.set.insert(value)
    }
}

#[derive(Debug)]
struct NonUniquePredicate<T> {
    set: HashSet<T>,
    allow_non_interesting: bool,
}

impl<T: Testable> NonUniquePredicate<T> {
    fn new(allow_non_interesting: bool) -> Self {
        NonUniquePredicate {
            set: Default::default(),
            allow_non_interesting,
        }
    }
}

impl<T: Testable> Predicate<T> for NonUniquePredicate<T> {
    #[inline]
    fn filter(&mut self, value: T) -> bool {
        self.allow_non_interesting && value.non_interesting() || !self.set.insert(value)
    }
}

impl<T: Testable> Predicate<T> for Either<UniquePredicate<T>, NonUniquePredicate<T>> {
    fn filter(&mut self, value: T) -> bool {
        match self.as_mut() {
            Either::Left(p) => p.filter(value),
            Either::Right(p) => p.filter(value),
        }
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let text = read_text(args)?;

    let mut predicate = initialize_predicate(args.invert, args.allow_blanks);
    let mut text: Vec<_> = text.lines().filter(|&s| predicate.filter(s)).collect();

    if args.sort {
        text.sort_by(|&a, &b| UniCase::new(a).cmp(&UniCase::new(b)));
    }

    if let Some(output) = args.output_path() {
        return write_to_file(output, text);
    }

    let mut out = io::stdout();
    for line in text {
        writeln!(out, "{line}")?;
    }

    Ok(())
}

fn write_to_file<'a>(path: &str, text: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
    let path = Path::new(path);
    let dir = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "parent directory not found"))?;

    let mut file = Builder::new().suffix(".u").tempfile_in(dir)?;
    let out = file.as_file_mut();

    for s in text {
        writeln!(out, "{s}")?;
    }

    fs::rename(file.path(), path)
}

fn read_text(args: &Args) -> io::Result<String> {
    match &args.path {
        Some(path) => fs::read_to_string(path),
        None => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn initialize_predicate<T: Testable>(invert: bool, allow_blanks: bool) -> impl Predicate<T> {
    if invert {
        Either::Right(NonUniquePredicate::new(allow_blanks))
    } else {
        Either::Left(UniquePredicate::new(allow_blanks))
    }
}
