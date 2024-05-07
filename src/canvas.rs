use crate::Color;
use std::cmp;
use std::io::{self, Write};

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

    pub fn write_ppm<T: Write>(&self, file: &mut T) -> io::Result<()> {
        // Write header.
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width(), self.height())?;
        writeln!(file, "255")?;
        // Write pixel data.
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self
                    .get_pixel(x, y)
                    .expect("the requested pixel should always be inside the canvas");
                let r = (255.0 * pixel.r).max(0.0).min(255.0);
                let g = (255.0 * pixel.g).max(0.0).min(255.0);
                let b = (255.0 * pixel.b).max(0.0).min(255.0);
                writeln!(file, "{:.0} {:.0} {:.0}", r, g, b)?;
            }
        }
        Ok(())
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
    use std::fs::File;
    use std::io::Read;
    use tempfile::NamedTempFile;

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

    #[test]
    fn save_canvas_to_ppm_file() -> io::Result<()> {
        // Open file handles.
        let mut file_w = NamedTempFile::new()?; // Write handle.
        let mut file_r: File = file_w.reopen()?; // Read handle.

        // Create canvas and setup pixel data.
        let mut canvas = Canvas::new(3, 2);
        canvas.set_pixel(0, 0, Color::red());
        canvas.set_pixel(1, 0, Color::green());
        canvas.set_pixel(2, 0, Color::blue());
        canvas.set_pixel(0, 1, Color::yellow());
        // Last two pixels check that clamping works correctly.
        let (r, g, b) = (1.1, 2.0, 1.0); // Should be clamped to white.
        canvas.set_pixel(1, 1, Color { r, g, b });
        let (r, g, b) = (0.0, -0.2, -1.0); // Should be clamped to black.
        canvas.set_pixel(2, 1, Color { r, g, b });

        // Check that canves is written correctly.
        canvas.write_ppm(&mut file_w)?;
        let mut buffer = String::new();
        file_r.read_to_string(&mut buffer)?;
        let mut lines = buffer.lines();
        assert_eq!(Some("P3"), lines.next());
        assert_eq!(Some("3 2"), lines.next());
        assert_eq!(Some("255"), lines.next());
        assert_eq!(Some("255 0 0"), lines.next()); // Red pixel at (0, 0).
        assert_eq!(Some("0 255 0"), lines.next()); // Green pixel at (1, 0).
        assert_eq!(Some("0 0 255"), lines.next()); // Blue pixel at (2, 0).
        assert_eq!(Some("255 255 0"), lines.next()); // Yellow pixel at (0, 1).
        assert_eq!(Some("255 255 255"), lines.next()); // White pixel at (1, 1).
        assert_eq!(Some("0 0 0"), lines.next()); // Black pixel at (2, 1).
        assert_eq!(None, lines.next()); // File should have ended.
        Ok(())
    }
}
