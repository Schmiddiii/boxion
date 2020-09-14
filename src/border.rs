/// A line around the box.
pub const LINED: Border = Border {
    n: '─',
    s: '─',
    e: '│',
    w: '│',
    ne: '┐',
    nw: '┌',
    se: '┘',
    sw: '└',
};

/// Only a space around the [Rect].
pub const SPACE: Border = Border {
    n: ' ',
    s: ' ',
    e: ' ',
    w: ' ',
    ne: ' ',
    nw: ' ',
    se: ' ',
    sw: ' ',
};

/// The style of a border.
#[derive(Clone)]
pub struct Border {
    pub n: char,
    pub s: char,
    pub e: char,
    pub w: char,
    pub ne: char,
    pub nw: char,
    pub se: char,
    pub sw: char,
}
