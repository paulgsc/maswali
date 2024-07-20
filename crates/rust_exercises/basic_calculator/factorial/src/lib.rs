pub fn factorial(n: u32) -> u32 {

    let res = if n < 2u32 {
        1u32
    } else {
        n * factorial(n - 1)
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
    fn third() {
        assert_eq!(factorial(2), 2);
    }
    
    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
} 
