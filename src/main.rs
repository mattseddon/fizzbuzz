use clap::Parser;
use std::{fmt, thread};

#[derive(Parser)]
struct Cli {
    iter_to: i64,
}

#[derive(PartialEq, Debug)]
enum Output {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i64),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Fizz => write!(f, "Fizz"),
            Output::Buzz => write!(f, "Buzz"),
            Output::FizzBuzz => write!(f, "FizzBuzz"),
            Output::Number(n) => write!(f, "{}", n),
        }
    }
}

fn fizz_buzz(n: i64) -> Output {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => Output::FizzBuzz,
        (true, false) => Output::Fizz,
        (false, true) => Output::Buzz,
        (false, false) => Output::Number(n),
    }
}

fn threaded_fizz_buzz(n: i64) -> Vec<Output> {
    let mut handles = Vec::new();
    for i in 1..n {
        let handle = thread::spawn(move || fizz_buzz(i));
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}

fn main() {
    let args = Cli::parse();

    println!("running fizz buzz up to {:?} in a sane loop", args.iter_to);

    for i in 1..args.iter_to {
        println!("{}", fizz_buzz(i));
    }

    println!("now do with threadz.... booooiiiiiiiiii");

    for i in threaded_fizz_buzz(args.iter_to) {
        println!("{}", i);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_not_fizz_or_buzz() {
        for i in 1..100 {
            if i % 3 == 0 || i % 5 == 0 {
                continue;
            }
            assert_eq!(fizz_buzz(i), Output::Number(i));
        }
    }

    #[test]
    fn test_fizz() {
        for i in 1..20 {
            if i % 5 == 0 {
                continue;
            }
            assert_eq!(fizz_buzz(i * 3), Output::Fizz);
        }
    }

    #[test]
    fn test_buzz() {
        for i in 1..20 {
            if i % 3 == 0 {
                continue;
            }

            assert_eq!(fizz_buzz(i * 5), Output::Buzz);
        }
    }

    #[test]
    fn test_fizz_buzz() {
        for i in 1..20 {
            assert_eq!(fizz_buzz(i * 3 * 5), Output::FizzBuzz);
        }
    }

    #[test]
    fn test_threaded_fizz_buzz() {
        let results = threaded_fizz_buzz(100);
        for i in 1..100 {
            assert_eq!(results[i - 1], fizz_buzz(i as i64));
        }
    }
}
