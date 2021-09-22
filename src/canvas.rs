use pixels::{Error, Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Canvas {
    pixels: Pixels,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Canvas, Error> {
        let pixels = {
            let surface_texture = SurfaceTexture::new(width, height, window);
            Pixels::new(width, height, surface_texture)?
        };

        Ok(Canvas {
            pixels,
            width,
            height,
        })
    }

    pub fn put_pixel(&mut self, pos: (i32, i32), color: (u8, u8, u8)) {
        let (x, y) = pos;
        let x = self.width as i32 / 2 + x;
        let y = self.height as i32 / 2 - y - 1;

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let offset = (self.width * 4 * y as u32 + 4 * x as u32) as usize;
        let buffer = self.pixels.get_frame();
        let (r, g, b) = color;
        buffer[offset..offset + 4].copy_from_slice(&[r, g, b, 255]);
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.pixels.render()
    }
}
