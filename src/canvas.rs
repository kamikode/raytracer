use crate::Color;
use std::fmt::{self, Debug};
use std::io::{self, Write};
use thiserror::Error;

const CLEAR_COLOR: Color = Color::black();

#[derive(Error)]
#[error("{0}")]
pub struct IndexError(String);

impl Debug for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Canvas<const W: usize, const H: usize> {
    data: [[Color; H]; W],
}

impl<const W: usize, const H: usize> Canvas<W, H> {
    pub fn new() -> Self {
        Canvas {
            data: [[CLEAR_COLOR; H]; W],
        }
    }

    pub fn width(&self) -> usize {
        W
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), IndexError> {
        match (x, y) {
            (_, _) if x < W && y < H => {
                self.data[x][y] = color;
                Ok(())
            }
            (_, _) => Err(IndexError(format!(
                "tried to set pixel at x={} and y={} \
                for canvas with width={} and height={} \
                (index out of bounds)",
                x, y, W, H
            ))),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        match (x, y) {
            (_, _) if x < W && y < H => Some(self.data[x][y]),
            (_, _) => None,
        }
    }

    pub fn write_ppm<T: Write>(&self, file: &mut T) -> Result<(), io::Error> {
        // Write header.
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width(), self.height())?;
        writeln!(file, "255")?;
        // Write pixel data.
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y).expect("indices should be valid");
                let r = (255.0 * pixel.r).max(0.0).min(255.0);
                let g = (255.0 * pixel.g).max(0.0).min(255.0);
                let b = (255.0 * pixel.b).max(0.0).min(255.0);
                writeln!(file, "{:.0} {:.0} {:.0}", r, g, b)?;
            }
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Default for Canvas<W, H> {
    fn default() -> Self {
        Canvas::new()
    }
}

impl<const W: usize, const H: usize> IntoIterator for Canvas<W, H> {
    type Item = Color;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[Color; H], W>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::<20, 10>::new();
        // Check that all 200 pixels are filled with the clear color.
        let mut count: usize = 0;
        for pixel in canvas {
            assert_eq!(pixel, CLEAR_COLOR);
            count += 1;
        }
        assert_eq!(count, 200);
    }

    #[test]
    fn width_and_height_are_correct() {
        let canvas = Canvas::<20, 10>::new();
        assert_eq!(canvas.width(), 20);
        assert_eq!(canvas.height(), 10);
    }

    #[test]
    fn set_and_get_pixel() -> Result<(), Box<dyn Error>> {
        let mut canvas = Canvas::<10, 20>::new();
        let red = Color::red();
        // Set pixel in-of-bounds and get it.
        canvas.set_pixel(2, 3, red)?;
        assert_eq!(canvas.get_pixel(2, 3).unwrap(), red);
        // Try to set pixel out-of-bounds.
        let e = canvas.set_pixel(5, 20, red).unwrap_err();
        assert_eq!(
            "tried to set pixel at x=5 and y=20 \
         for canvas with width=10 and height=20 \
         (index out of bounds)",
            e.0
        );
        // Try to get pixel out-of-bounds.
        assert_eq!(canvas.get_pixel(10, 5), None);
        Ok(())
    }

    #[test]
    fn save_canvas_to_ppm_file() -> Result<(), Box<dyn Error>> {
        // Open file handles.
        let mut file_w = NamedTempFile::new()?; // Write handle.
        let mut file_r: File = file_w.reopen()?; // Read handle.

        // Create canvas and setup pixel data.
        let mut canvas = Canvas::<3, 2>::new();
        canvas.set_pixel(0, 0, Color::red())?;
        canvas.set_pixel(1, 0, Color::green())?;
        canvas.set_pixel(2, 0, Color::blue())?;
        canvas.set_pixel(0, 1, Color::yellow())?;
        // Last two pixels check that clamping works correctly.
        let (r, g, b) = (1.1, 2.0, 1.0); // Should be clamped to white.
        canvas.set_pixel(1, 1, Color { r, g, b })?;
        let (r, g, b) = (0.0, -0.2, -1.0); // Should be clamped to black.
        canvas.set_pixel(2, 1, Color { r, g, b })?;

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
