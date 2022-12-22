// In excercises 1 through 3, assume that A and B are finite sets of integers. Write a sburoutine
// to compute the specified set.

// Exercise 1: A ∪ B
fn union<T: Eq + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut ans = a.to_owned();
    for i in b {
        if ans.contains(i) {continue}
        ans.push(*i);
    }
    ans
}
// Exercise 2: A ∩ B
fn intersection<T: Eq + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut ans = Vec::<T>::new();
    for element in b {
        if a.contains(element) {ans.push(*element)}
    }
    ans
}
// Exercise 3: A - B
fn complement<T: Eq + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut ans = Vec::<T>::new();
    for element in a {
        if !b.contains(element) {ans.push(*element)}
    }
    ans
}

// Exercise 4: Consider the sequence recursively defined by g(0) = 1 , g(1) = -1,
// g(n) = 3g(n-1) + 2g(n-2).
fn g(n: i128) -> i128 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return -1;
    }
    3*g(n-1) + 2*g(n-2)
}
// (a) write a subroutne that will print the first 20 terms of the sequence.
fn first_20() -> Vec<i128> {
    let mut ans: Vec<i128> = Vec::new();
    for n in 0..20 {
        ans.push(g(n));
    }
    ans
}
// (b) write a subroutine that will print the first n terms of the sequence. The user should be
// able to supply the value of n at runtime.
fn first_n(n: i128) -> Vec<i128> {
    let mut ans: Vec<i128> = Vec::new();
    for n in 0..n {
        ans.push(g(n));
    }
    ans
}

// Exercise 5: Write a subroutine to find the least common multiple of two positive integers.
fn gcd(x: u32, y: u32) -> u32 {
    let mut x = x;
    let mut y = y;
    while x != y {
        if x > y {
            x = x - y;
        }
        else {
            y = y - x;
        }
    }
    x
}
fn lcm(x: u32, y: u32) -> u32 {
    let gcd = gcd(x,y);
    x*y/gcd
}

// test module for the exercises
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![3, 4, 5, 6, 7];
        let c = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(union(&a, &b), c);
    }

    #[test]
    fn test_exercise_2() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![3, 4, 5, 6, 7];
        let c = vec![3, 4, 5];
        assert_eq!(intersection(&a, &b), c);
    }

    #[test]
    fn test_exercise_3() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![3, 4, 5, 6, 7];
        let c = vec![1, 2];
        assert_eq!(complement(&a, &b), c);
    }

    #[test]
    fn test_exercise_4_a() {
        let seq = first_20();
        assert_eq!(seq, vec![1, -1, -1, -5, -17, -61, -217, -773, -2753, -9805, -34921, -124373, -442961, -1577629, -5618809, -20011685, -71272673, -253841389, -904069513, -3219891317]);
    }

    #[test]
    fn test_exercise_4_b() {
        let seq = first_n(10);
        assert_eq!(seq, vec![1, -1, -1, -5, -17, -61, -217, -773, -2753, -9805]);
    }

    #[test]
    fn test_exercise_5() {
        assert_eq!(lcm(12, 18), 36);
    }
}