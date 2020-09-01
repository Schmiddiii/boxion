/// Represents a rectangle. All values are given as percentages.
#[derive(Clone)]
pub struct Rect {
    pub x: u8,
    pub y: u8,

    pub width: u8,
    pub height: u8,

    pub split: Option<Box<Split>>,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            split: None,
        }
    }
}

/**
Represents a split.
The first Rect in rects is the left/top item
*/
#[derive(Clone)]
pub struct Split {
    rects: (Rect, Rect),
    percentage: f32,
    direction: SplitDirection,
    has_border: bool
}

#[derive(Clone)]
pub enum SplitDirection {
    VERTICAL,
    HORIZONTAL,
}

impl Rect {
    pub fn split(&mut self, direction: SplitDirection, percentage: f32, has_border: bool) {
        match direction {
            SplitDirection::HORIZONTAL => {
                let r1 = Rect {
                    x: self.x,
                    y: self.y,
                    width: (self.width as f32 * percentage) as u8,
                    height: self.height,
                    split: None,
                };
                let r2 = Rect {
                    x: self.x + r1.width + (has_border as u8),
                    y: self.y,
                    width: self.width - r1.width - (has_border as u8),
                    height: self.height,
                    split: None,
                };

                self.split = Some(Box::new(Split {
                    rects: (r1, r2),
                    percentage: percentage,
                    direction: direction,
                    has_border: has_border
                }));
            }
            SplitDirection::VERTICAL => {
                let r1 = Rect {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: (self.height as f32 * percentage) as u8,
                    split: None,
                };
                let r2 = Rect {
                    x: self.x,
                    y: self.y + r1.height + (has_border as u8),
                    width: self.width,
                    height: self.height - r1.height - (has_border as u8),
                    split: None,
                };

                self.split = Some(Box::new(Split {
                    rects: (r1, r2),
                    percentage: percentage,
                    direction: direction,
                    has_border: has_border
                }));
            }
        }
    }

    pub fn mod_split(&mut self, f: fn((Rect, Rect)) -> (Rect, Rect)) {
        let spl = self.get_split();
        if spl.is_none() {
            return;
        }

        self.split.unwrap().rects = f(self.split.clone().unwrap().rects);

    }

    pub fn get_split(&self) -> Option<(Rect, Rect)> {
        if self.split.is_none() {
            return None;
        }

        return Some(self.split.as_ref().unwrap().rects.clone());

    }
}


