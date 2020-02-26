use std::collections::{hash_map, HashMap, HashSet};

pub fn get_alphabet(words: &[&str]) -> Vec<char> {
    let mut unprocessed_chars: HashSet<_> = words.iter().flat_map(|w| w.chars()).collect();

    // no letters for the alphabet
    if unprocessed_chars.is_empty() {
        return vec![];
    }

    // pre-allocate one less than the number of characters
    // if there are 2 characters, then only one has any characters ahead
    // and only one has any characters before
    let mut before_chars = HashMap::with_capacity(unprocessed_chars.len() - 1);
    let mut after_chars = HashMap::with_capacity(unprocessed_chars.len() - 1);

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
                // but the number of characters before is unknown (cap at max chars - 1)
                .or_insert(HashSet::new())
                .insert(right);
            after_chars
                .entry(right)
                // we could pre-allocate this set with a capacity
                // but the number of characters after is unknown (cap at max chars - 1)
                .or_insert(HashSet::new())
                .insert(left);
        });

    let mut alphabet = Vec::with_capacity(unprocessed_chars.len());

    while !unprocessed_chars.is_empty() {
        // only one char left to process
        // it must be the last one
        if unprocessed_chars.len() == 1 {
            // this unwrap is safe because we know there is exactly one character in the set
            alphabet.push(unprocessed_chars.into_iter().next().unwrap());
            break;
        }

        // find the (hopefully only) character that has no characters before it
        let next_lowest = *unprocessed_chars
            .iter()
            .find(|c| !after_chars.contains_key(c))
            // all characters have a latter constraint
            // this means that the constraints have a conflict
            // OR there is a bug
            .expect("ambiguous input");

        before_chars
            .remove(&next_lowest)
            .unwrap_or_default()
            .into_iter()
            .for_each(|c| {
                // remove the constraints for all the characters
                // that had 'next_lowest' as a constraint
                if let hash_map::Entry::Occupied(mut entry) = after_chars.entry(c) {
                    entry.get_mut().remove(&next_lowest);
                    if entry.get().is_empty() {
                        entry.remove();
                    }
                }
            });

        // done processing this character
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
