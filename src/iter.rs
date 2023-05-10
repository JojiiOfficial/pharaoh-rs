use crate::{
    alignment::{AlignMapping, Alignment},
    error::PharaohError,
};
use std::{collections::HashMap, str::Split};

pub struct StringAligner<'s, 'd, 'a> {
    alignment: Split<'a, char>,
    src: &'s str,
    dst: &'d str,
    split: char,
}

impl<'s, 'd, 'a> StringAligner<'s, 'd, 'a> {
    #[inline]
    pub(crate) fn new(alignment: &'a str, src: &'s str, dst: &'d str, split: char) -> Self {
        Self {
            alignment: alignment.trim().split(' '),
            src,
            dst,
            split,
        }
    }

    /// Parses the alignments into a HashMap.
    pub fn to_map(self) -> Result<HashMap<&'s str, Vec<&'d str>>, PharaohError> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

        for alignment in self {
            let alignment = alignment?;
            map.entry(alignment.src())
                .or_default()
                .push(alignment.dst());
        }

        Ok(map)
    }

    fn next_align(&mut self) -> Result<Option<Alignment>, PharaohError> {
        let next = match self.alignment.next() {
            Some(s) => s,
            None => return Ok(None),
        };

        if next.ends_with('-') {
            return Err(PharaohError::InvalidAlignmentFormat);
        }

        let center = match next.find('-') {
            Some(c) => c,
            None => return Err(PharaohError::InvalidAlignmentFormat),
        };

        let src_idx: usize = next[..center]
            .parse()
            .map_err(|_| PharaohError::InvalidAlignmentFormat)?;

        let dst_idx: usize = next[center + 1..]
            .parse()
            .map_err(|_| PharaohError::InvalidAlignmentFormat)?;

        Ok(Some(Alignment::new(src_idx, dst_idx)))
    }
}

impl<'s, 'd, 'a> Iterator for StringAligner<'s, 'd, 'a> {
    type Item = Result<AlignMapping<'s, 'd>, PharaohError>;

    fn next(&mut self) -> Option<Self::Item> {
        let align = match self.next_align() {
            Ok(v) => v?,
            Err(err) => return Some(Err(err)),
        };

        let src_sub = match self.src.split(self.split).nth(align.src()) {
            Some(d) => d,
            None => return Some(Err(PharaohError::IndexOutOfBonuds(align.src()))),
        };

        let dst_sub = match self.dst.split(self.split).nth(align.dst()) {
            Some(d) => d,
            None => return Some(Err(PharaohError::IndexOutOfBonuds(align.dst()))),
        };

        Some(Ok(AlignMapping::new(src_sub, dst_sub)))
    }
}
