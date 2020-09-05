use crate::border;
use crate::layout;

use std::collections::HashMap;
use std::io::{Stdout, Write};
use std::io::{Error, ErrorKind};

/// A rectangle on the terminal
#[derive(Clone)]
pub struct Rect {
    pub name: String,
    position: Position,
    has_border: bool,
    border: Option<border::Border>,
}

/// Position information for the rectangle
#[derive(Clone)]
struct Position {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Rect {
    /// Converts a layout to a [HashMap] of [Rect].
    /// The key of the [HashMap] will be the name chosen in the [Layout.new] or similar functions.
    pub fn from_layout(
        layout: &layout::Layout,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> HashMap<String, Rect> {
        if layout.split.is_none() && layout.name.is_some() {
            let mut hm = HashMap::new();
            hm.insert(
                layout.name.clone().unwrap(),
                Rect {
                    name: layout.name.clone().unwrap(),
                    position: Position {
                        x: x+(layout.has_border as u16),
                        y: y+(layout.has_border as u16),
                        width: width-2*(layout.has_border as u16),
                        height: height-2*(layout.has_border as u16),
                    },
                    has_border: layout.has_border,
                    border: layout.border.clone(),
                },
            );

            return hm;
        } else if layout.split.is_some() {
            let split = layout.split.clone().unwrap();

            let (r1x, r1y, r1w, r1h);
            let (r2x, r2y, r2w, r2h);

            match split.direction {
                layout::SplitDirection::HORIZONTAL => {
                    r1x = x;
                    r1y = y;
                    r1w = (width as f32 * split.percentage) as u16;
                    r1h = height;

                    r2x = x + r1w;
                    r2y = y;
                    r2w = width - r1w;
                    r2h = height;
                }

                layout::SplitDirection::VERTICAL => {
                    r1x = x;
                    r1y = y;
                    r1w = width;
                    r1h = (height as f32 * split.percentage) as u16;

                    r2x = x;
                    r2y = y + r1h;
                    r2w = width;
                    r2h = height - r1h;
                }
            }

            let mut hm1 = Rect::from_layout(&split.rects.0.clone(), r1x, r1y, r1w, r1h);
            let hm2 = Rect::from_layout(&split.rects.1.clone(), r2x, r2y, r2w, r2h);

            hm1.extend(hm2);

            return hm1;
        }

        return HashMap::new();
    }

    /// Shows the writeable box of the [Rect] on the screen.
    pub fn show(&self, stdout: &mut Stdout) {
        let pos = self.position.clone();
        write!(stdout, "{}┌", termion::cursor::Goto(pos.x, pos.y)).unwrap();
        write!(
            stdout,
            "{}┐",
            termion::cursor::Goto(pos.x + pos.width - 1, pos.y)
        )
        .unwrap();
        write!(
            stdout,
            "{}└",
            termion::cursor::Goto(pos.x, pos.y + pos.height - 1)
        )
        .unwrap();
        write!(
            stdout,
            "{}┘",
            termion::cursor::Goto(pos.x + pos.width - 1, pos.y + pos.height - 1)
        )
        .unwrap();
    }

    // Shows the [Border] of the [Rect] if existent.
    pub fn show_border(&self, stdout: &mut Stdout) {
        if !self.has_border || self.border.is_none() {
            return;
        }
        let border = self.border.clone().unwrap();
        let pos = self.position.clone();

        for i in 0..(pos.height + 1) {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(pos.x - 1, pos.y + i),
                border.w
            )
            .unwrap();

            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(pos.x + pos.width, pos.y + i),
                border.e
            )
            .unwrap();
        }

        for i in 0..(pos.width + 1) {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(pos.x + i, pos.y - 1),
                border.n
            )
            .unwrap();

            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(pos.x + i, pos.y + pos.height),
                border.s
            )
            .unwrap();
        }

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(pos.x - 1, pos.y - 1),
            border.nw
        )
        .unwrap();
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(pos.x + pos.width, pos.y - 1),
            border.ne
        )
        .unwrap();
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(pos.x - 1, pos.y + pos.height),
            border.sw
        )
        .unwrap();
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(pos.x + pos.width, pos.y + pos.height),
            border.se
        )
        .unwrap();
    }

    /// Write a string to the screen at the given positions.
    /// The positinos are (0,0)-based.
    /// If the string is to long to fit in the line it will be wrapped.
    /// If the string will not fit in the border a [Error] will be returned and nothing will be written.
    pub fn write(&self, stdout: &mut Stdout, str: &str, x: u16, y: u16) -> Result<(), Error> {
        if y>self.position.height {
            return Err(Error::new(ErrorKind::Other,"Position out of bounds"));
        }

        let overflow = (x as i16) + (str.len() as i16) - (self.position.width as i16);

        // Overflow to next line
        if overflow > 0 {
            let thisline = &str[..(str.len()-overflow as usize)];
            let nextline = &str[(str.len()-overflow as usize)..];
            self.write(stdout, thisline, x, y).unwrap();
            return self.write(stdout, nextline, 0, y+1);
        }

        // No overflow
        return write!(stdout, "{}{}", termion::cursor::Goto(self.position.x+x, self.position.y+y), str);
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        return (self.position.width, self.position.height);
    }
}