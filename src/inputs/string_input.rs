// pest. Smart PEGs in Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::Input;

/// A `struct` useful for matching in-memory `String`s.
///
/// # Examples
///
/// ```
/// # use pest::Input;
/// # use pest::StringInput;
/// let mut input = StringInput::new("asdasdf");
///
/// assert!(input.matches("asd"));
/// assert!(input.matches("asdf"));
/// assert!(!input.matches("nope"));
/// ```
pub struct StringInput {
    string: String,
    pos: usize
}

impl StringInput {
    /// Creates a new `StringInput` from a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Input;
    /// # use pest::StringInput;
    /// let mut input = StringInput::new("asd");
    ///
    /// assert_eq!(input.len(), 3);
    /// ```
    pub fn new(string: &str) -> StringInput {
        StringInput {
            string: string.to_owned(),
            pos : 0
        }
    }
}

impl Input for StringInput {
    fn len(&self) -> usize {
        self.string.len()
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }

    fn matches(&mut self, string: &str) -> bool {
        let mut skipped = self.string.chars().skip(self.pos);

        for c1 in string.chars() {
            match skipped.next() {
                Some(c2) => {
                    if c1 != c2 {
                        return false
                    }
                },
                None => {
                    return false
                }
            }
        }

        self.pos += string.len();

        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Input;
    use super::StringInput;

    #[test]
    fn empty() {
        let mut input = StringInput::new("");

        assert!(input.matches(""));
        assert!(!input.matches("a"));
    }

    #[test]
    fn parts() {
        let mut input = StringInput::new("asdasdf");

        assert!(input.matches("asd"));
        assert!(input.matches("asdf"));
    }

    #[test]
    fn len() {
        assert_eq!(StringInput::new("asdasdf").len(), 7);
    }

    #[test]
    fn pos() {
        let mut input = StringInput::new("asdasdf");

        assert_eq!(input.pos(), 0);
        assert!(input.matches("asd"));
        assert_eq!(input.pos(), 3);
        assert!(input.matches("asdf"));
        assert_eq!(input.pos(), 7);

        input.set_pos(3);

        assert_eq!(input.pos(), 3);
        assert!(input.matches("asdf"));
        assert_eq!(input.pos(), 7);
    }
}