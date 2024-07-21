pub fn factorial(n: u32) -> u32 {
    let res = if n < 2 {
        1u32
    } else {
        let mut carry = n; 
        let mut curr = carry;

        while curr > 1 {
            curr -= 1u32;
            carry *= curr;
        };

        carry
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

