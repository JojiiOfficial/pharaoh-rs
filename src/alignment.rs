/// A single alignment mapping between two sentences
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alignment {
    src: usize,
    dst: usize,
}

impl Alignment {
    #[inline]
    pub fn new(src: usize, dst: usize) -> Self {
        Self { src, dst }
    }

    #[inline]
    pub fn src(&self) -> usize {
        self.src
    }

    #[inline]
    pub fn dst(&self) -> usize {
        self.dst
    }
}

/// A single alignment mapping between two sentences
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlignMapping<'s, 'd> {
    src: &'s str,
    dst: &'d str,
}

impl<'s, 'd> AlignMapping<'s, 'd> {
    #[inline]
    pub fn new(src: &'s str, dst: &'d str) -> Self {
        Self { src, dst }
    }

    #[inline]
    pub fn src(&self) -> &'s str {
        self.src
    }

    #[inline]
    pub fn dst(&self) -> &'d str {
        self.dst
    }
}
