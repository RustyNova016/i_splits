use std::fmt::Write;

pub trait ISplitExt {
    /// Split the string in half at the occurence `i` of the pattern.
    ///
    /// `i` starts at one.
    ///
    /// # Panics
    /// Panics if the provided indice is 0
    ///
    /// # Exemples
    /// ```
    /// use i_splits::ISplitExt as _;
    ///
    /// let v = "To show you the power of i_split, I cut that sentence in half!".split_i(", ", 1);
    /// assert_eq!(v, Some(("To show you the power of i_split".to_string(), "I cut that sentence in half!".to_string())));
    ///
    /// let v = "cookie|lolipop|muffin|pancake".split_i("|", 2);
    /// assert_eq!(v, Some(("cookie|lolipop".to_string(), "muffin|pancake".to_string())));
    ///
    /// let v = "No splits? That's a `None` for you".split_i("!", 2);
    /// assert_eq!(v, None);
    ///
    /// let v = "Don't go too far either!".split_i(" ", 10);
    /// assert_eq!(v, None);
    /// ```
    fn split_i(&self, pat: &str, i: usize) -> Option<(String, String)>;

    /// Split the string in half at the last occurence of the pattern.
    ///
    /// # Exemples
    /// ```
    /// use i_splits::ISplitExt as _;
    ///
    /// let v = "To show you the power of i_split, I cut that sentence in half!".split_once_last(", ");
    /// assert_eq!(v, Some(("To show you the power of i_split".to_string(), "I cut that sentence in half!".to_string())));
    ///
    /// let v = "cookie|lolipop|muffin|pancake".split_once_last("|");
    /// assert_eq!(v, Some(("cookie|lolipop|muffin".to_string(), "pancake".to_string())));
    /// ```
    fn split_once_last(&self, pat: &str) -> Option<(String, String)>;
}

impl ISplitExt for str {
    fn split_i(&self, pat: &str, i: usize) -> Option<(String, String)> {
        assert_ne!(0, i);
        let mut iter = self.split(pat);
        let Some(left) = iter.next() else {
            unreachable!("Splits without any matches return the whole string")
        };
        let mut left = left.to_string();
        let mut counter = 1usize;

        while counter < i {
            if let Some(chunk) = iter.next() {
                write!(left, "{}{}", pat, chunk).unwrap();
                counter += 1
            } else {
                // Not enough splits to split at requested indice. We exit.
                return None;
            }
        }

        // We arrived at the requested indice! We merge back the rest of the iterator
        let mut right = String::new();
        for chunk in iter {
            if counter == i {
                // Special write to not include the patern at the start of the split's right
                write!(right, "{}", chunk).unwrap();
                counter += 1
            } else {
                write!(right, "{}{}", pat, chunk).unwrap();
            }
        }

        Some((left, right))
    }

    fn split_once_last(&self, pat: &str) -> Option<(String, String)> {
        let (i, _) = self.matches(pat).enumerate().last()?;

        self.split_i(pat, i + 1)
    }
}
