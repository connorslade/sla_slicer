use common::serde::{Deserializer, Serializer};

pub struct PreviewImage<const WIDTH: usize, const HEIGHT: usize> {
    // 0brrrrrggggggbbbbb
    data: Box<[u16]>,
}

impl<const WIDTH: usize, const HEIGHT: usize> PreviewImage<WIDTH, HEIGHT> {
    pub fn empty() -> Self {
        let mut data = vec![0; WIDTH * HEIGHT];

        for (i, pixel) in data.iter_mut().enumerate() {
            *pixel = u16::MAX * (i % 2 == 0) as u16;
        }

        Self {
            data: data.into_boxed_slice(),
        }
    }

    pub fn inner_data(&self) -> &[u16] {
        &self.data
    }

    pub fn serializes<T: Serializer>(&self, serializer: &mut T) {
        for pixel in self.data.iter() {
            serializer.write_u16(*pixel);
        }
    }

    pub fn deserializes(deserializer: &mut Deserializer) -> Self {
        let mut out = Self::empty();

        for pixel in out.data.iter_mut() {
            *pixel = deserializer.read_u16();
        }

        out
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (f32, f32, f32)) {
        let red = (color.0 * 31.0).round() as u16;
        let green = (color.1 * 63.0).round() as u16;
        let blue = (color.2 * 31.0).round() as u16;
        self.data[y * WIDTH + x] = (red << 11) | (green << 5) | blue;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> (f32, f32, f32) {
        let pixel = self.data[y * WIDTH + x];
        let red = ((pixel >> 11) & 0x1F) as f32 / 31.0;
        let green = ((pixel >> 5) & 0x3F) as f32 / 63.0;
        let blue = (pixel & 0x1F) as f32 / 31.0;
        (red, green, blue)
    }
}
