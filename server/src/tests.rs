macro_rules! assert_strings {
    ($str: literal, $func: expr) => {
        let a: &str = $str;
        let b: String = String::from($str);
        let c: &String = &b;

        let fmt = |t: &str| {
            format!(
                "callable {} with param of type {} failed",
                stringify!($func),
                t
            )
        };

        #[allow(clippy::redundant_closure_call)]
        let a_result = $func(a);
        #[allow(clippy::redundant_closure_call)]
        let c_result = $func(c);
        #[allow(clippy::redundant_closure_call)]
        let b_result = $func(b);

        assert!(a_result, "{}", fmt("$str"));
        assert!(c_result, "{}", fmt("&String"));
        assert!(b_result, "{}", fmt("String"));
    };
}

pub(crate) use assert_strings;
