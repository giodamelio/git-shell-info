named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::parens;

    #[test]
    fn it_works() {
        assert_eq!(parens(b"(aaa)"), IResult::Done(&b""[..], &b"aaa"[..]));
    }
}
