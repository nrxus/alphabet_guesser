use std::collections::{hash_map, HashMap, HashSet};

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

struct Alphabet {
    /// characters grouped by all the characters we know come before it
    prior_constraints: HashMap<char, HashSet<char>>,
    /// characters grouped by all the characters we know come after it
    latter_constraints: HashMap<char, HashSet<char>>,
    /// all characters
    characters: HashSet<char>,
}

impl Alphabet {
    /// words that are sorted based on the unknown Alphabet
    /// returns None if there are no characters in the Alphabet
    /// otherwise returns Some(Alphabet)
    pub fn new(words: &[&str]) -> Option<Self> {
        let characters: HashSet<_> = words.iter().flat_map(|w| w.chars()).collect();

        if characters.is_empty() {
            None
        } else {
            Some(Alphabet::new_unchecked(words, characters))
        }
    }

    /// returns an `Iterator<Item = char>`
    /// that drains the smallest character out of the alphabet on every iteration
    pub fn drain(&mut self) -> impl Iterator<Item = char> + '_ {
        AlphabetDrain { alphabet: self }
    }

    /// asssumes the character set is NOT empty and it matches all the characters in words
    fn new_unchecked(words: &[&str], characters: HashSet<char>) -> Self {
        let mut latter_constraints = HashMap::with_capacity(characters.len() - 1);
        let mut prior_constraints = HashMap::with_capacity(characters.len() - 1);

        words
            .windows(2)
            .flat_map(|pair| {
                let left = pair[0];
                let right = pair[1];

                // find the first character where they differ
                // to use as a constraint
                left.chars()
                    .zip(right.chars())
                    .find(|(left, right)| left != right)
            })
            .for_each(|(left, right)| {
                // add `right` to the set of characters latter to `left`
                latter_constraints
                    .entry(left)
                    // we could pre-allocate this set with a capacity
                    // but the number of characters before is unknown (cap at max chars - 1)
                    .or_insert(HashSet::new())
                    .insert(right);

                // add `left` to the set of characters prior to `right`
                prior_constraints
                    .entry(right)
                    // we could pre-allocate this set with a capacity
                    // but the number of characters after is unknown (cap at max chars - 1)
                    .or_insert(HashSet::new())
                    .insert(left);
            });

        Alphabet {
            latter_constraints,
            prior_constraints,
            characters,
        }
    }
}

/// Drain for `Alphabet`
///
/// It implements Iterator<Item = char> such that on every "next" character
/// it pops the "smallest" character from the stored alphabet
struct AlphabetDrain<'a> {
    alphabet: &'a mut Alphabet,
}

impl<'a> Iterator for AlphabetDrain<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let alphabet = &mut self.alphabet;

        // cannot continue iterating; no more characters
        if alphabet.characters.is_empty() {
            return None;
        }

        let next_lowest = if alphabet.characters.len() == 1 {
            // only a single character left so it must be the smallest
            *alphabet.characters.iter().next().unwrap()
        } else {
            // find the (hopefully only) character that has no characters prior to it
            let next_lowest = *alphabet
                .characters
                .iter()
                .find(|c| !alphabet.prior_constraints.contains_key(c))
                // all characters have a prior contraint
                // this means that there is a conflict
                // OR there is a bug
                .expect("ambiguous input");

            alphabet
                .latter_constraints
                .remove(&next_lowest)
                .unwrap_or_default()
                .into_iter()
                .for_each(|c| {
                    // remove the constraints for all the characters
                    // that had 'next_lowest' as a prior_constraint
                    if let hash_map::Entry::Occupied(mut entry) =
                        alphabet.prior_constraints.entry(c)
                    {
                        entry.get_mut().remove(&next_lowest);
                        if entry.get().is_empty() {
                            entry.remove();
                        }
                    }
                });

            next_lowest
        };

        alphabet.characters.remove(&next_lowest);
        Some(next_lowest)
    }

    /// A hint on how long this iterator might run for
    /// Used to pre-allocate when collecting into a Vec
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.alphabet.characters.len(),
            Some(self.alphabet.characters.len()),
        )
    }
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
