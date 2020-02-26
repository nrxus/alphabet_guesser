use std::collections::{hash_map, HashMap, HashSet};

pub fn get_alphabet(words: &[&str]) -> Vec<char> {
    let mut unprocessed_chars: HashSet<_> = words.iter().flat_map(|w| w.chars()).collect();

    // pre-allocate
    let mut before_chars: HashMap<char, _> = HashMap::new();
    let mut after_chars: HashMap<char, _> = HashMap::new();

    words
        .windows(2)
        .flat_map(|pair| {
            let left = pair[0];
            let right = pair[1];

            left.chars()
                .zip(right.chars())
                .find(|(left, right)| left != right)
        })
        .for_each(|(left, right)| {
            before_chars
                .entry(left)
                // we could pre-allocate this set with a capacity
                .or_insert(HashSet::new())
                .insert(right);
            after_chars
                .entry(right)
                // we could pre-allocate this set with a capacity
                .or_insert(HashSet::new())
                .insert(left);
        });

    // pre-allocate
    let mut alphabet = vec![];

    while !unprocessed_chars.is_empty() {
        if unprocessed_chars.len() == 1 {
            alphabet.push(unprocessed_chars.into_iter().next().unwrap());
            break;
        }

        let next_lowest = *unprocessed_chars
            .iter()
            .find(|c| !after_chars.contains_key(c))
            .expect("ambiguous input");

        before_chars
            .remove(&next_lowest)
            .unwrap_or_default()
            .into_iter()
            .for_each(|c| {
                if let hash_map::Entry::Occupied(mut entry) = after_chars.entry(c) {
                    entry.get_mut().remove(&next_lowest);
                    if entry.get().is_empty() {
                        entry.remove();
                    }
                }
            });

        unprocessed_chars.remove(&next_lowest);
        alphabet.push(next_lowest);
    }

    alphabet
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
}
