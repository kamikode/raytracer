use crate::Color;
use std::cmp;

const CLEAR_COLOR: Color = Color::black();

#[derive(Debug)]
pub struct Canvas {
    data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        match (width, height) {
            (_, 0) => Canvas {
                data: vec![vec![CLEAR_COLOR; 0]; 0],
            },
            (0, _) => Canvas {
                data: vec![vec![CLEAR_COLOR; 0]; 0],
            },
            _ => Canvas {
                data: vec![vec![CLEAR_COLOR; height]; width],
            },
        }
    }

    pub fn width(&self) -> usize {
        self.data.len()
    }

    pub fn height(&self) -> usize {
        let res = self.data.first();
        match res {
            Some(col) => col.len(),
            None => 0,
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_height != self.height() {
            for row in &mut self.data {
                row.resize(new_height, CLEAR_COLOR);
            }
        }
        if new_width != self.width() {
            self.data.resize(new_width, vec![CLEAR_COLOR; new_height]);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let new_width = cmp::max(self.width(), x + 1);
        let new_height = cmp::max(self.height(), y + 1);
        self.resize(new_width, new_height);
        self.data[x][y] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color> {
        let col = self.data.get(x);
        match col {
            Some(col) => col.get(y),
            None => None,
        }
    }
}

impl IntoIterator for Canvas {
    type Item = Color;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<Vec<Color>>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);
        // Check that all 200 pixels are filled with the clear color.
        let mut count: usize = 0;
        for pixel in canvas {
            assert_eq!(pixel, CLEAR_COLOR);
            count += 1;
        }
        assert_eq!(count, 200);
    }

    #[test]
    fn width_is_correct() {
        let canvas = Canvas::new(20, 10);
        assert_eq!(canvas.width(), 20);
    }

    #[test]
    fn height_is_correct() {
        let canvas = Canvas::new(20, 10);
        assert_eq!(canvas.height(), 10);
    }

    #[test]
    fn create_zero_width_canvas() {
        let canvas = Canvas::new(0, 10);
        assert_eq!(canvas.width(), 0);
        assert_eq!(canvas.height(), 0);
    }

    #[test]
    fn create_zero_height_canvas() {
        let canvas = Canvas::new(10, 0);
        assert_eq!(canvas.width(), 0);
        assert_eq!(canvas.height(), 0);
    }

    #[test]
    fn resize() {
        let mut canvas = Canvas::new(10, 20);
        canvas.resize(11, 20);
        assert_eq!(canvas.width(), 11);
        assert_eq!(canvas.height(), 20);
        canvas.resize(11, 15);
        assert_eq!(canvas.width(), 11);
        assert_eq!(canvas.height(), 15);
    }

    #[test]
    fn set_and_get_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::red();
        // Set pixel in-of-bounds and get it.
        canvas.set_pixel(2, 3, red);
        assert_eq!(*canvas.get_pixel(2, 3).unwrap(), red);
        // Trying to access out-of-bounds pixels.
        assert_eq!(canvas.get_pixel(10, 5), None);
        assert_eq!(canvas.get_pixel(5, 20), None);
        assert_eq!(canvas.get_pixel(10, 20), None);
    }

    #[test]
    fn set_pixel_expands_canvas_automatically() {
        // Expand width.
        let mut canvas = Canvas::new(0, 1);
        canvas.set_pixel(0, 0, CLEAR_COLOR);
        assert_eq!(canvas.width(), 1);
        // Expand height.
        let mut canvas = Canvas::new(1, 0);
        canvas.set_pixel(0, 0, CLEAR_COLOR);
        assert_eq!(canvas.height(), 1);
        // Expand width + height.
        let mut canvas = Canvas::new(0, 0);
        canvas.set_pixel(0, 0, CLEAR_COLOR);
        assert_eq!(canvas.width(), 1);
        assert_eq!(canvas.height(), 1);
    }
}
