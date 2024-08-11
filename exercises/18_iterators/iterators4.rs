fn factorial(num: u8) -> u64 {
    // TODO: Complete this function to return the factorial of `num`.
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    //Nota personal: te dejo otra alternativa por si sirve. Para no olvidar su funcionamiento
    //std::ops::Range {start : 1, end : (num as u64) + 1}.fold(1, |acc, n| acc * n)
    std::ops::Range {start : 1, end : (num as u64) + 1}.product::<u64>() //product multiplica todos los elementos de un interador y devuelve el producto
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
