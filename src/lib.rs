pub fn get_alphabet(_: &[&str]) -> Vec<char> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(get_alphabet(&[]), vec![]);
    }
}
