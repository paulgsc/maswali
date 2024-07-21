pub fn factorial(n: u32) -> u32 {
    let res = if n < 2 {
        1
    } else {
        let mut f = n;
        for i in 1..n {
            f *= i;
        }

        f
    };

    res
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn thrid() {
        assert_eq!(factorial(2), 2);
    }
    
    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
