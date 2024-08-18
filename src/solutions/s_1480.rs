fn solution(input: Vec<i32>) -> Vec<i32> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inital_test() {
        let solution = solution(vec![1, 2, 3, 4]);
        assert_eq!(vec![1, 3, 6, 10], solution);
    }
}
