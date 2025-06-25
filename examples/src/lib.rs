
#[cfg(test)]
mod code_with_declarative {
    use declarative::foreach;

    #[test]
    #[allow(unused_variables)]
    pub fn should_perform_option_foreach() {
        let r: Option<i32> = foreach! {
            a <- (Some(1))
            b <- (None::<i32>) if (a != 2)
            c <- (Some(3))
            yield a + b + c
        };

        assert_eq!(r, None)
    }
}

#[cfg(test)]
mod code_with_procedural {
    use procedural::foreach;

    #[test]
    pub fn should_perform_lists_foreach_with_filter() {
        let r: Option<i32> = foreach! {
            a <- Some(1)
            b <- None::<i32> if a != 2
            c <- Some(3)
            yield a + b + c
        };

        assert_eq!(r, None)
    }
}
