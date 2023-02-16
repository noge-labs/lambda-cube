#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub index: u32,
}

impl Pos {
    #[inline]
    pub fn new(index: u32) -> Pos {
        Pos { index }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Pos,
    pub end: Pos,
}

impl Range {
    #[inline]
    pub fn new(start: Pos, end: Pos) -> Range {
        Range { start, end }
    }

    #[inline]
    pub fn mix(&self, end: Range) -> Range {
        Range {
            start: self.start.clone(),
            end: end.end,
        }
    }
}
