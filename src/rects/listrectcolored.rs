use crate::rect::Rect;
use std::io::Write;
use std::cmp;

/// Uses a rect to make a list of items. These items can be selected.
pub struct ListRectColored<'a, T> {
    rect: Rect,
    elements: Vec<(T, String)>,
    selected: usize,
    theme: &'a dyn Fn(
        T,
        bool,
    ) -> (
        &'static dyn termion::color::Color,
        &'static dyn termion::color::Color,
    ),
}

impl<'a, T: Clone> ListRectColored<'a, T> {
    /// Clears the rect
    pub fn clear(&self, stdout: &mut dyn Write) {
        self.rect.clear(stdout);
    }

    /// Set the elements
    pub fn set_elements(&mut self, elements: Vec<(T, String)>) {
        self.elements = elements;
        self.selected = 0;
    }

    /// Change one single element in the list
    pub fn set_element(&mut self, index: usize, element: (T, String)) {
        if index >= self.elements.len() {
            return;
        }
        self.elements[index] = element;
    }

    /// Stets the theme.
    /// The theme is a function that takes the extra information given with each item and wheather it is selected
    /// and returns the foreground color and background color
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

    /// Get the string of the currently selected item.
    pub fn get_selected(&self) -> Option<String> {
        if self.elements.len() == 0 || self.selected >= self.elements.len() {
            return None;
        }
        return Some(self.elements[self.selected].1.clone());
    }

    /// Get the extra information of the currently selected item.
    pub fn get_selected_extra(&self) -> Option<T> {
        if self.elements.len() == 0 || self.selected >= self.elements.len() {
            return None;
        }
        return Some(self.elements[self.selected].0.clone());
    }

    /// Show the list.
    pub fn show(&self, stdout: &mut dyn Write) {
        self.rect.show_border(stdout);

        let offset = cmp::max(0, self.selected as isize - self.rect.get_dimensions().1 as isize + 1);

        for (index, element) in self.elements.iter().enumerate() {
            if index < offset as usize {
                continue;
            }
            if index >= offset as usize + self.rect.get_dimensions().1 as usize {
                return;
            }

            let (fg_color, bg_color) =
                (self.theme)(element.0.clone(), index == self.selected);
            self.rect
                .write_colored_trimmed_line(stdout, &element.1, index as u16 - offset as u16, &*fg_color, &*bg_color)
                .unwrap();
        }
    }

    /// Get the currently selected index.
    pub fn get_index(&self) -> usize {
        return self.selected.clone();
    }

    /// Moves the cursur to the next element. The selection will wrap around.
    pub fn next(&mut self) {
        self.selected = (self.selected as i32 + 1).rem_euclid(self.elements.len() as i32) as usize;
    }

    /// Move the cursor to the previous element. The selection will wrap around.
    pub fn prev(&mut self) {
        self.selected = (self.selected as i32 - 1).rem_euclid(self.elements.len() as i32) as usize;
    }
}

impl crate::rect::Rect {
    /// Creates a list from the rect.
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
