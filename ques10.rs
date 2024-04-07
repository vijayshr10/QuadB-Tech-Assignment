fn is_prime(n: u64) -> bool {
    if n <= 1 {
      return false; // 1 or less are not prime
    }
    if n <= 3 {
      return true; // 2 and 3 are prime exceptions
    }
    if n % 2 == 0 || n % 3 == 0 {
      return false; // Divisible by 2 or 3 means not prime
    }
  
    let mut i = 5;
    while i * i <= n {
      if n % i == 0 || n % (i + 2) == 0 {
        return false; // Divisible by a specific odd number or its plus-two counterpart means not prime
      }
      i += 6; // Increment by 6 to check only odd divisors efficiently
    }
  
    true // If no divisors found within the loop, the number is prime
  }
  
  fn main() {
    let number = 11;
    if is_prime(number) {
      println!("{} is a prime number", number);
    } else {
      println!("{} is not a prime number", number);
    }
  }
  