# Alphabet Guesser
A library to guess the order of an alphabet given a list of words
sorted by that alphabet

## Setup

This project is a Rust library. The easiest way to install Rust is
through [rustup].

[rustup]: https://rustup.rs/

## Running Tests

`cargo test`

## Reading External Docs

This library is not published but you can read the docs locally by
doing:

`cargo doc --open`

This will open a local version of the docs in your browser.

## High Level Overview

This algorithm has two distinctive steps:

1. Gather all the constraints

   This is done through the construction of `Alphabet`.

   1. Get a set of all the characters in all the words
   2. Loop through all the words in pairs and keep track of what
      characters come before/after another

2. Use the constraints to repeatedly find the "first" character

   This is done by "draining" the `Alphabet`. Everytime we drain the
   alphabet we:

   1. Go through all the characters in the alphabet and find what
      character did not come after any other characters. We had stored
      this information in the first step
   2. Remove the character found to come first from the list of
      characters in the alphabet, and unconstrain any character that
      comes after it
   3. Return this first character

   Everytime the drain iterates, it keeps draining the lowest
   character, leaving a new lowest behind. We do this until there is
   nothing left in the alphabet to drain.

This implementation focuses on simplicity of the algorithm rather than
performance. Thought was given on keeping track of the order in
distinct linked lists that get merged somehow later and that might
have been a more efficient approach but it would come at a high degree
of extra complexity that I would avoid unless this is critical code
that has been perf tested to show that it a current bottleneck.

The time complexity of this algorithm is

```
O(nm + c^2)

n: number of words
m: number of characters in the second largest word
c: number of characters in the alphabet
```

The memory complexity is:

```
O(c^2)

c: number of characters in the alphabet
```

## Caveats

Sorting graphemes with multiple codepoints is left unspecified. While
this solution works for unicode characters, regardless of byte size,
it assumes that they all fit in one "code point". For example the
grapheme "नी" is actually two code points: "न" and "ी". A library can
be pulled in to handle this case, and the code is ready to be migrated
to it with minor changes. This work is left undone because it is
unclear what a "character" could mean in the context of sorting an
alphabet, but with more knowledge about user needs this case can also
be handled.
