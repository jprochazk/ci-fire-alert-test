pub fn add(left: usize, right: usize) -> usize {
    // failing clippy lint
    if left == left {
        return left * 2;
    }

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failing_test() {
        assert_eq!(add(2, 2), 5);
    }
}
