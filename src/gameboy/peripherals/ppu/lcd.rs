use sdl2::{Sdl, pixels::PixelFormatEnum, render::Canvas, video::Window};

const LCD_WIDTH: u32 = 160;
const LCD_HEIGHT: u32 = 144;

pub struct Lcd {
    canvas: Canvas<Window>,
}

impl Lcd {
    pub fn new(sdl: &Sdl, scale: u32) -> Lcd {
        let video = sdl
            .video()
            .expect("failed to initialize SDL video subsystem");
        let window = video
            .window("gb-emu", LCD_WIDTH * scale, LCD_HEIGHT * scale)
            .position_centered()
            .resizable()
            .build()
            .expect("failed to create a window");
        let canvas = window
            .into_canvas()
            .build()
            .expect("failed to create canvas");
        Self { canvas }
    }

    pub fn draw(&mut self, pixels: Box<[u8]>) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .expect("failed to create texture streaming");
        texture
            .update(None, &pixels, 480)
            .expect("failed to update texture");
        self.canvas.clear();
        self.canvas
            .copy(&texture, None, None)
            .expect("failed to copy canvas");
        self.canvas.present();
    }
}
