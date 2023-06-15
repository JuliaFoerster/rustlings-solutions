// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut v = 1..=num;
    v.fold(1,|accum, current| accum*current)

     //   accum * current
     //   f(x) = x+1
     //   |x| = x*(x-1)
     //   0 = 0 *-1 = 0 OK
     //   1 = 1*0 = 0 FALSE 1
     //   2 = 2*1 = 2 OK 2
     //   3 = 3*2 = 6 OK 6
     //   4 = 4*3 = 12 FALSE 24
     //   5 = 5*4 = 20 FALSE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
