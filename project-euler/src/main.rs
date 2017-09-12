fn main() {
    let answer_1 = euler_1();
    println!("{}", answer_1);

    let answer_2 = euler_2();
    println!("{}", answer_2);

    let answer_3 = euler_3();
    println!("{}", answer_3);

    let answer_7 = euler_7();
    println!("{}", answer_7);

    let answer_35 = euler_35();
    println!("{}", answer_35);
}

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
// Answer 233168
fn euler_1() -> i32 {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 {
            sum += i;
        }
        else if i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
// Answer 4613732
fn euler_2() -> i32 {
    let mut v = vec![1,2];

    let mut sum: i32 = v.iter().rev().take(2).sum();
    while sum < 4000000 {
        v.push(sum);
        sum = v.iter().rev().take(2).sum();
    }

    let answer: i32 = v.into_iter().filter(|x| x % 2 == 0).sum();
    answer
}


// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
// Answer 6857
// NB this is not fast enough to check the real key space should look into profiling
fn euler_3() -> i64 {
    let upper: i64 = 600851475143 / 10000;

    for i in (2..upper).rev() {

        if 600851475143 % i == 0 {
            let mut is_prime = true;

            for j in 2..i {
                if i % j == 0 {
                    is_prime = false;
                }
            }

            if is_prime {
                if 600851475143 % i == 0 {
                    return i
                }
            }
        }
    }

    0
}

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?
// Answer 104743
fn euler_7() -> i32 {
    
    let mut count = 0;
    let mut i = 2;

    loop {
        let mut is_prime = true;

        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            count += 1;
        }

        if count == 10001 {
            break;
        }

        i += 1;
    }

    i
}

// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
// How many circular primes are there below one million?
fn euler_35() -> i32 {
    let mut count = 0;

    for i in 1..1000001 {
        if filter_circular(i) {
            if is_prime(i) {
                let mut rot = rotate_number(i);
                let mut is_circular = true;

                for _ in 1..i.to_string().len() {
                    if !is_prime(rot) {
                        is_circular = false;
                    }
                    rot = rotate_number(rot);
                }

                if is_circular {
                    count += 1;
                }
            }
        }
    }

    count
}

fn filter_circular(x: i32) -> bool {
    if x == 2 {
        return true;
    }

    let string = x.to_string();

    if string.contains("2") || string.contains("4") || string.contains("5") || string.contains("6") || string.contains("8") || string.contains("0") {
        return false;
    }
    true
}

fn is_prime(x: i32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false
        }
    }

    true
}

fn rotate_number(x: i32) -> i32 {
    let string = x.to_string();

    // Warning: Not unicode aware
    let start = &string[1..string.len()];
    let end = &string[0..1];
    let toreturn = format!("{}{}", start, end);

    let ret: i32 = toreturn.parse().unwrap();
    ret
}


#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_euler_1() {
         assert_eq!(233168, euler_1());
    }

    #[test]
    fn test_euler_2() {
         assert_eq!(4613732, euler_2());
    }

    #[test]
    fn test_euler_3() {
         assert_eq!(6857, euler_3());
    }

    #[test]
    fn test_euler_7() {
         assert_eq!(104743, euler_7());
    }

    #[test]
    fn test_euler_35() {
         assert_eq!(55, euler_35());
    }
}


