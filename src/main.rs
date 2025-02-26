use clap::Parser;
use std::thread;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    iter_to: i64,
}

fn fizz_buzz(n: i64) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        return n.to_string();
    }
}

fn make_parallel(n: i64) -> Vec<String> {
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

    let parallel = make_parallel(args.iter_to);
    for i in parallel {
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
            assert_eq!(fizz_buzz(i), i.to_string());
        }
    }

    #[test]
    fn test_fizz() {
        for i in 1..20 {
            if i % 5 == 0 {
                continue;
            }
            assert_eq!(fizz_buzz(i * 3), "Fizz");
        }
    }

    #[test]
    fn test_buzz() {
        for i in 1..20 {
            if i % 3 == 0 {
                continue;
            }

            assert_eq!(fizz_buzz(i * 5), "Buzz");
        }
    }

    #[test]
    fn test_fizz_buzz() {
        for i in 1..20 {
            assert_eq!(fizz_buzz(i * 3 * 5), "FizzBuzz");
        }
    }

    #[test]
    fn test_make_parallel() {
        let results = make_parallel(100);
        for i in 1..20 {
            assert_eq!(results[i - 1], fizz_buzz(i as i64));
        }
    }
}
