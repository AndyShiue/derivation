#[macro_use]
extern crate derivation;
extern crate enum_variants;

#[cfg(test)]
mod tests {

    use enum_variants::Variants;
    use self::Enum::*;

    #[derive(Debug, PartialEq, Eq, FromStr, Variants)]
    enum Enum {
        Variant1,
        Variant2,
        Variant3,
    }

    #[test]
    fn variants() {
        assert_eq!(Enum::variants(), vec![Variant1, Variant2, Variant3])
    }

    #[test]
    fn from_str_pass() {
        assert_eq!(Variant1, "Variant1".parse().unwrap())
    }

    #[test]
    #[should_panic]
    fn from_str_fail() {
        assert_eq!(Variant1, "Variant4".parse().unwrap())
    }

}
