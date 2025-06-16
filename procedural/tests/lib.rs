#[cfg(test)]
pub mod procedural_foreach_tests {
    use procedural::foreach;

    #[test]
    fn should_perform_lists_foreach_with_filter() {
        let code: Vec<_> = foreach! {
            a <- (1..=3)
            b <- (1..=a) if (b != 2)
            yield (a,b)
        }
        .collect();

        assert_eq!(code, vec![(1, 1), (2, 1), (3, 1), (3, 3)]);
    }

    #[test]
    pub fn should_perform_option_foreach() {
        let r: Option<i32> = foreach! {
            a <- Some(1)
            b <- None::<i32>
            c <- Some(3)
            yield a + b + c
        };

        assert_eq!(r, None)
    }
}
