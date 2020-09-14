extern crate termion;

use crate::border;

/// Represents a rectangular layout.
#[derive(Clone)]
pub struct Layout {
    pub(crate) name: Option<String>,
    pub(crate) split: Option<Box<Split>>,
    pub(crate) has_border: bool,
    pub(crate) border: Option<border::Border>,
}

/// Represents a split of a [Layout].
#[derive(Clone)]
pub(crate) struct Split {
    pub(crate) rects: (Layout, Layout),
    pub(crate) split_type: SplitType,
    pub(crate) direction: SplitDirection,
}

/// Represents the type of the split.
/// The split can either be done with percentages (using f32) of
/// by given the amount of rows/columns reserved by one of the [Layout]s.
/// If given a positive number the first one will have the wanted amount of rows/columns
/// otherwise the second one.
#[derive(Clone)]
pub(crate) enum SplitType {
    PERCENTAGE(f32),
    VALUE(i16),
}

/// Represents in what direction to split.
/// A vertical split means that the boxes will be vertical and the split horizontal.
#[derive(Clone)]
pub enum SplitDirection {
    VERTICAL,
    HORIZONTAL,
}

impl Layout {
    /// Create a new [Layout] with a given name as [&str]
    pub fn new_str(name: &str) -> Layout {
        return Layout::new(String::from(name));
    }

    /// Create a new [Layout] with a given name as [String]
    pub fn new(name: String) -> Layout {
        return Layout {
            split: None,
            name: Some(name),
            has_border: false,
            border: None,
        };
    }

    /// Create a new [Layout] with a given name as [&str] and a [Border].
    pub fn new_border_str(name: &str, border: border::Border) -> Layout {
        return Layout::new_border(String::from(name), border);
    }

    /// Create a new [Layout] with a given name as [String] and a [Border].
    pub fn new_border(name: String, border: border::Border) -> Layout {
        return Layout {
            split: None,
            name: Some(name),
            has_border: true,
            border: Some(border),
        };
    }

    /// Merges two [Layout] using percentages. The new layout will be returned
    pub fn merge(r1: Layout, r2: Layout, direction: SplitDirection, percentage: f32) -> Layout {
        let has_border = r1.has_border || r2.has_border;
        Layout {
            name: None,
            split: Some(Box::new(Split {
                rects: (r1, r2),
                split_type: SplitType::PERCENTAGE(percentage),
                direction: direction,
            })),
            has_border: has_border,
            border: None,
        }
    }

    /// Merges two [Layout] using a specified value for the rows/columns. The new layout will be returned
    pub fn merge_value(r1: Layout, r2: Layout, direction: SplitDirection, value: i16) -> Layout {
        let has_border = r1.has_border || r2.has_border;
        Layout {
            name: None,
            split: Some(Box::new(Split {
                rects: (r1, r2),
                split_type: SplitType::VALUE(value),
                direction: direction,
            })),
            has_border: has_border,
            border: None,
        }
    }
}
