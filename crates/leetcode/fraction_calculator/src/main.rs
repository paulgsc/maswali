use std::str::FromStr;
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
        Fraction { numerator, denominator }
    }
    
    fn add(&self, other: &Fraction) -> Fraction {
        let new_numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let new_denominator = self.denominator * other.denominator;
        Fraction::new(new_numerator, new_denominator)
    }
    
    fn substract(&self, other: &Fraction) -> Fraction {
        let new_numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        let new_denominator = self.denominator * other.denominator;
        Fraction::new(new_numerator, new_denominator)
    }

    fn simplify(&mut self) {
        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        self.numerator /= gcd;
        self.denominator /= gcd;
        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }
}

impl FromStr for Fraction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let numerator = parts[0].parse().map_err(|_| ())?;
        let denominator = parts[1].parse().map_err(|_| ())?;
        Ok(Fraction::new(numerator, denominator))
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn calculate_fraction(expression: &str) -> String {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    let mut result = Fraction::from_str(tokens[0]).unwrap();


    for chunk in tokens[1..].chunks(2) {
        if chunk.len() != 2 {
            break;
        }
        let operator = chunk[0];
        let fraction = Fraction::from_str(chunk[1]).unwrap();


        match operator {
            "+" => result = result.add(&fraction),
            "-" => result = result.substract(&fraction),
            _ => panic!("Invalid operator"),
        }
    }

    result.simplify();

    if result.denominator == 1 {
        format!("{}/1", result.numerator)
    } else {
        format!("{}/{}", result.numerator, result.denominator)
    }
}


fn main() {
    println!("Welcome to the Fraction Calculator!");
    println!("Enter a fraction expression (e.g, '1/3 + 1/2 - 1/4') or 'quit' to exit:");


    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();


        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();


        let input = input.trim();

        if input.to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }

        match calculate_fraction(input) {
            result => println!("Result: {}", result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate_fraction("1/3 + 1/3"), "2/3");
    }

    #[test]
    fn test_substraction() {
        assert_eq!(calculate_fraction("1/2 - 1/3"), "1/6");
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(calculate_fraction("1/3 - 1/2 + 1/3"), "1/6");
    }

    #[test]
    fn test_simplication() {
        assert_eq!(calculate_fraction("4/6 + 2/3"), "4/3");
    }

    #[test]
    fn test_integer_result() {
        assert_eq!(calculate_fraction("1/3 + 2/3"), "1/1");
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(calculate_fraction("1/3 - 1/2"), "-1/6");
    }
}

