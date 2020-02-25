pub fn get_alphabet(words: &[&str]) -> Vec<char> {
    words.iter().map(|w| w.chars().next().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(get_alphabet(&[]), vec![]);
    }

    #[test]
    fn single_letter() {
        assert_eq!(
            get_alphabet(&["a", "b", "c", "d"]),
            vec!['a', 'b', 'c', 'd']
        );
    }
}
