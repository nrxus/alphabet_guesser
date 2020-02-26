mod alphabet;

use alphabet::Alphabet;

/// returns a list of characters for a given alphabet
/// that has the same sort order as the list of words passed in the input
pub fn get_alphabet(words: &[&str]) -> Vec<char> {
    let mut alphabet = match Alphabet::new(words) {
        // no alphabet could be created; no characters
        None => return vec![],
        Some(alphabet) => alphabet,
    };

    // drain every character in the alphabet
    // and collect it into a Vec<char>
    alphabet.drain().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_words() {
        assert_eq!(get_alphabet(&[]), vec![]);
    }

    #[test]
    fn single_letter() {
        assert_eq!(
            get_alphabet(&["a", "b", "c", "d"]),
            vec!['a', 'b', 'c', 'd']
        );
    }

    #[test]
    fn empty_word() {
        assert_eq!(get_alphabet(&[""]), vec![]);
    }

    #[test]
    fn ties_in_first_character() {
        assert_eq!(get_alphabet(&["bac", "aaa", "acb"]), vec!['b', 'a', 'c']);
    }

    #[test]
    fn more_complex() {
        assert_eq!(
            get_alphabet(&["zzzzzz", "zzzzq", "t", "qqzq", "qqz5", "55", "5b", "rr", "rbb", "rbf"]),
            vec!['z', 't', 'q', '5', 'r', 'b', 'f']
        );
    }

    #[test]
    fn unicode() {
        assert_eq!(get_alphabet(&["źęń", "ęęę", "ęńź"]), vec!['ź', 'ę', 'ń']);
    }
}
