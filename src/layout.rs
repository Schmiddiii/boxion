extern crate termion;

use std::io::{Stdout, Write};
// use termion::cursor;

/// Represents a rectangle. All values are given as percentages.
#[derive(Clone)]
pub struct Rect {
    pub split: Option<Box<Split>>,
    pub position: Option<Position>,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            split: None,
            position: None,
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
    has_border: bool,
}

#[derive(Clone)]
pub struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

#[derive(Clone)]
pub enum SplitDirection {
    VERTICAL,
    HORIZONTAL,
}

impl Rect {
    pub fn merge_with(
        &mut self,
        other: Rect,
        direction: SplitDirection,
        percentage: f32,
        has_border: bool,
    ) {
        self.split = Some(Box::new(Split {
            rects: ((*self).clone(), other),
            percentage: percentage,
            has_border: has_border,
            direction: direction,
        }));
    }

    pub fn update_position(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.position = Some(Position {
            x: x,
            y: y,
            width: width,
            height: height,
        });

        if self.split.is_none() {
            return;
        }

        let split : &mut std::boxed::Box<Split> = self.split.as_mut().unwrap();

        match split.direction {
            SplitDirection::HORIZONTAL => {
                let (r1x, r1y, r1w, r1h) = (x, y, (width as f32 * split.percentage) as u16, height);
                let (r2x, r2y, r2w, r2h) = (
                    x + r1w + (split.has_border as u16) + 1,
                    y,
                    width - r1w - (split.has_border as u16) - 1,
                    height,
                );

                split.rects.0.update_position(r1x, r1y, r1w, r1h);
                split.rects.1.update_position(r2x, r2y, r2w, r2h);
            }

            SplitDirection::VERTICAL => {
                let (r1x, r1y, r1w, r1h) = (x, y, width, (height as f32 * split.percentage) as u16);
                let (r2x, r2y, r2w, r2h) = (
                    x,
                    y + r1h + (split.has_border as u16) + 1,
                    width,
                    height - r1h - (split.has_border as u16) - 1,
                );

                split.rects.0.update_position(r1x, r1y, r1w, r1h);
                split.rects.1.update_position(r2x, r2y, r2w, r2h);
            }
        }
    }

    pub fn debug_print(&mut self, stdout: &mut Stdout, x: u16, y: u16) -> u16 {
        write!(stdout, "{}", termion::cursor::Goto(x,y)).unwrap();
        if self.split.is_none() {
            match self.position.as_ref() {
                None => write!(stdout, "Final").unwrap(),
                Some(pos) => write!(stdout, "Final @ {} {}", pos.x, pos.y).unwrap()
            }
            return y+1;
        } else {
            match (self.position.as_ref(), self.split.as_mut().unwrap().direction.clone()) {
                (None, SplitDirection::HORIZONTAL) => write!(stdout, "HORIZONTAL Split").unwrap(),
                (None, SplitDirection::VERTICAL) => write!(stdout, "VERTICAL Split").unwrap(),
                (Some(pos), SplitDirection::HORIZONTAL) => write!(stdout, "HORIZONTAL Split @ {} {}", pos.x, pos.y).unwrap(),
                (Some(pos), SplitDirection::VERTICAL) => write!(stdout, "VERTICAL Split @ {} {}", pos.x, pos.y).unwrap()
            }
            let y2 = self.split.as_mut().unwrap().rects.0.debug_print(stdout, x+1, y+1);
            let y3 = self.split.as_mut().unwrap().rects.1.debug_print(stdout, x+1, y2);
            return y3;
        }

    }

    pub fn show(&self, stdout: &mut Stdout) {
        if self.split.is_some() {
            let (r1, r2) = &self.split.as_ref().unwrap().rects;
            r1.show(stdout);
            r2.show(stdout);
        } else if self.position.is_some() {
            let pos = self.position.as_ref().unwrap();
            write!(stdout, "{}┌", termion::cursor::Goto(pos.x, pos.y)).unwrap();
            write!(stdout, "{}┐	", termion::cursor::Goto(pos.x+pos.width, pos.y)).unwrap();
            write!(stdout, "{}└", termion::cursor::Goto(pos.x, pos.y+pos.height)).unwrap();
            write!(stdout, "{}┘", termion::cursor::Goto(pos.x+pos.width, pos.y+pos.height)).unwrap();
        }
    }
}

