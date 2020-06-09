use indicatif::ParallelProgressIterator;
use primes::{PrimeSet, Sieve};
use progressive::progress;
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::Write;
use std::{fs, io};
use crate::VisitResult::{AlreadySaw, WasUnseen};

struct Ring {
    n: u64,
    seen: HashSet<u64>,
    last: u64
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum VisitResult {
    WasUnseen,
    AlreadySaw
}

impl Ring {
    fn with_size(size: u64) -> Ring {
        Ring {
            n: size,
            seen: HashSet::new(),
            last: 0
        }
    }

    fn visit(&mut self, n: u64) -> VisitResult {
        let n_actual = (self.last + n) % self.n;
        if self.seen.contains(&n_actual) {
            AlreadySaw
        } else {
            self.seen.insert(n_actual);
            self.last = n_actual;
            WasUnseen
        }
    }
}

#[derive(Debug)]
struct RunResult {
    seen: u64,
    unseen: u64,
}

impl RunResult {
    fn proportion(&self) -> f64 {
        self.seen as f64 / (self.seen + self.unseen) as f64
    }
}
fn run_ring(size: u64, pattern: impl Iterator<Item = u64>) -> RunResult {
    let mut ring = Ring::with_size(size);
    let mut seen = 0;
    for item in pattern {
        if ring.visit(item) == WasUnseen {
            seen += 1;
        } else {
            break;
        }
    }
    RunResult {
        seen,
        unseen: size - seen,
    }
}

fn main() -> io::Result<()> {
    println!("Starting computation");
    let range: Vec<u64> = (3u64..1000u64).collect();
    let results: Vec<(u64, RunResult)> = range
        .par_iter()
        .progress_count(range.len() as u64)
        .map(|size| (*size, run_ring(*size, Sieve::new().iter())))
        .collect();

    println!("Writing to file");
    let mut outfile = fs::File::create("output.csv")?;
    outfile.write("size,seen,unseen,proportion\n".as_bytes())?;
    for (size, res) in progress(results.iter()) {
        outfile.write(
            format!(
                "{},{},{},{}\n",
                size,
                res.seen,
                res.unseen,
                res.proportion()
            )
            .as_bytes(),
        )?;
    }
    outfile.flush()?;

    println!("Finding maximum fill");
    let (n, max) = progress(results.iter())
        .max_by(|(_, result1), (_, result2)| {
            result1
                .proportion()
                .partial_cmp(&result2.proportion())
                .unwrap()
        })
        .unwrap();
    println!("MAX: {:?}: {:?}", n, max);

    Ok(())
}
