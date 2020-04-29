
use glium::texture::Texture2d;
use glium::texture::RawImage2d;
use glium::Display;

pub struct Screen {
    pub width : f32,
    pub height : f32,
}

impl Screen {
    pub fn new(width : f32, height : f32) -> Screen {
        Screen { width, height }
    }

    pub fn texture(&self, display : &Display) -> Texture2d {
        let image = RawImage2d::from_raw_rgba_reversed(&[0u8, 200, 0, 0,
                                                        0, 200, 200, 0,
                                                        0, 0, 200, 0,
                                                        0, 0, 0, 0 ], (2, 2));
        Texture2d::new(display, image).unwrap()
    }
}
