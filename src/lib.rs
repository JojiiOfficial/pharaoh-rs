pub mod alignment;
pub mod error;
pub mod iter;

use iter::StringAligner;

/// Pharaoh parser.
pub struct Pharaoh {
    word_sep: char,
}

impl Pharaoh {
    /// Create a new default Pharaoh parser.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Use a custom separator for words.
    pub fn with_word_sep(mut self, sep: char) -> Self {
        self.word_sep = sep;
        self
    }

    /// Aligns two sentences by string.
    pub fn align_by_str<'s, 'd, 'a>(
        &self,
        src: &'s str,
        dst: &'d str,
        align_str: &'a str,
    ) -> StringAligner<'s, 'd, 'a> {
        StringAligner::new(align_str, src, dst, self.word_sep)
    }
}

impl Default for Pharaoh {
    #[inline]
    fn default() -> Self {
        Self { word_sep: ' ' }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::alignment::AlignMapping;

    #[test]
    fn test_align_by_str() {
        let src = "this is a test";
        let dst = "これ は テスト です";
        let align = "0-0 1-1 1-3 3-2";

        let mut align = Pharaoh::new().align_by_str(src, dst, align);
        assert_eq!(align.next(), Some(Ok(AlignMapping::new("this", "これ"))));
        assert_eq!(align.next(), Some(Ok(AlignMapping::new("is", "は"))));
        assert_eq!(align.next(), Some(Ok(AlignMapping::new("is", "です"))));
        assert_eq!(align.next(), Some(Ok(AlignMapping::new("test", "テスト"))));
        assert_eq!(align.next(), None);
    }

    #[test]
    fn test_align_by_str_leading_space() {
        let src = "this is a test";
        let dst = "これ は テスト です";
        let align = "0-0 1-1 1-3 3-2 ";

        let c = Pharaoh::new().align_by_str(src, dst, align).count();
        assert_eq!(c, 4);
    }
}
