use crate::rect::Rect;
use std::io::Write;


pub struct ListTheme {
    pub selected_fg: Box<dyn termion::color::Color>,
    pub selected_bg: Box<dyn termion::color::Color>,
}

pub struct ListRect {
    rect: Rect,
    elements: Vec<String>,
    selected: u16,
    theme: ListTheme,
}

impl ListRect {

    pub fn clear(&self, stdout: &mut dyn Write) {
        self.rect.clear(stdout);
    }

    pub fn set_elements(&mut self, elements: Vec<String>) {
        self.elements = elements;
        self.selected = 0;
    }

    pub fn set_theme(&mut self, theme: ListTheme) {
        self.theme = theme;
    }

    pub fn get_selected(&self) -> String {
        return self.elements[self.selected as usize].clone();
    }

    pub fn show(&self, stdout: &mut dyn Write) {
        self.rect.show_border(stdout);
        for (index, element) in self.elements.iter().enumerate() {
            if index as u16 >= self.rect.get_dimensions().1 {
                break;
            }
            if index as u16 != self.selected {
                self.rect.write_trimmed(stdout, element, 0, index as u16).unwrap();
            } else {
                self.rect.write_colored_trimmed(stdout, element, 0, index as u16, &*self.theme.selected_fg, &*self.theme.selected_bg).unwrap();
            }
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected as i32 + 1).rem_euclid(self.elements.len() as i32) as u16;
    }

    pub fn prev(&mut self) {
        self.selected = (self.selected as i32 - 1).rem_euclid(self.elements.len() as i32) as u16;
    }

}

impl crate::rect::Rect {
    pub fn into_list(self) -> Box<ListRect> {
        return Box::new(ListRect {
            rect: self,
            elements: vec![],
            selected: 0,
            theme: ListTheme {
                selected_fg: Box::new(termion::color::Rgb(0, 0, 0)),
                selected_bg: Box::new(termion::color::White),
            },
        });
    }
}
