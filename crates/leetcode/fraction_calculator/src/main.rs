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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("invalid fraction format".to_string());
        }
        let numerator = parts[0].parse().map_err(|_| "Invalid numerator")?;
        let denominator = parts[1].parse().map_err(|_| "Invalid denominator")?;
        if denominator == 0 {
            return Err("Denominator cannot be zero".to_string());
        }
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
    let mut result = Fraction::new(0, 1);
    let mut current_fraction = String::new();
    let mut current_sign = 1;

    for (i, c) in expression.chars().enumerate() {
        match c {
            '0' ..='9' | '/' => current_fraction.push(c),
            '+' | '-' => {
                if !current_fraction.is_empty() || i == 0 {
                    if let Ok(fraction) = Fraction::from_str(&current_fraction) {
                        let signed_fraction = Fraction::new(fraction.numerator * current_sign, fraction.denominator);
                        result = result.add(&signed_fraction);
                        current_fraction.clear();
                    }
                }
                current_sign = if c == '+' { 1 } else { -1 };
            },
            _ => return "Invalid character in expression".to_string(),
        }
    }
    


    if !current_fraction.is_empty() {
        if let Ok(fraction) = Fraction::from_str(&current_fraction) {
            let signed_fraction = Fraction::new(fraction.numerator * current_sign, fraction.denominator);
            result = result.add(&signed_fraction);
        }
    }

    result.simplify();

    if result.numerator == 0 {
        "0/1".to_string()
    } else if result.numerator > 0 {
        format!("{}/{}", result.numerator, result.denominator)
    } else {
        format!("-{}/{}", result.numerator.abs(), result.denominator)
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
        assert_eq!(calculate_fraction("1/3+1/3"), "2/3");
    }

    #[test]
    fn test_substraction() {
        assert_eq!(calculate_fraction("1/2-1/3"), "1/6");
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(calculate_fraction("1/3-1/2+1/3"), "1/6");
    }

    #[test]
    fn test_simplication() {
        assert_eq!(calculate_fraction("4/6+2/3"), "4/3");
    }

    #[test]
    fn test_integer_result() {
        assert_eq!(calculate_fraction("1/3+2/3"), "1/1");
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(calculate_fraction("1/3-1/2"), "-1/6");
    }
}

