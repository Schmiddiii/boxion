use crate::rect::Rect;
use std::io::Write;

pub struct ListRectColored<'a, T> {
    rect: Rect,
    elements: Vec<(T, String)>,
    selected: u16,
    theme: &'a dyn Fn(
        T,
        bool,
    ) -> (
        &'static dyn termion::color::Color,
        &'static dyn termion::color::Color,
    ),
}

impl<'a, T: Clone> ListRectColored<'a, T> {
    pub fn clear(&self, stdout: &mut dyn Write) {
        self.rect.clear(stdout);
    }

    pub fn set_elements(&mut self, elements: Vec<(T, String)>) {
        self.elements = elements;
        self.selected = 0;
    }

    pub fn set_theme(
        &mut self,
        theme: &'a dyn Fn(
            T,
            bool,
        ) -> (
            &'static dyn termion::color::Color,
            &'static dyn termion::color::Color,
        ),
    ) {
        self.theme = theme;
    }

    pub fn get_selected(&self) -> Option<String> {
        if self.elements.len() == 0 || self.selected as usize >= self.elements.len() {
            return None;
        }
        return Some(self.elements[self.selected as usize].1.clone());
    }

    pub fn show(&self, stdout: &mut dyn Write) {
        self.rect.show_border(stdout);
        for (index, element) in self.elements.iter().enumerate() {
            let (fg_color, bg_color) =
                (self.theme)(element.0.clone(), index as u16 == self.selected);
            self.rect
                .write_colored_trimmed(stdout, &element.1, 0, index as u16, &*fg_color, &*bg_color)
                .unwrap();
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
    pub fn into_list_colored<'a, T>(
        self,
        theme: &'a dyn Fn(
            T,
            bool,
        ) -> (
            &'static dyn termion::color::Color,
            &'static dyn termion::color::Color,
        ),
    ) -> Box<ListRectColored<'a, T>> {
        return Box::new(ListRectColored::<T> {
            rect: self,
            elements: vec![],
            selected: 0,
            theme: theme,
        });
    }
}
