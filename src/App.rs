use std::{error::Error, rc::Rc, thread, time::Duration};

use sdl2::{
    EventPump, Sdl,
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    ttf::{self, Font},
};

pub struct App {
    sdl_context: Sdl,
    canvas: Canvas<sdl2::video::Window>,
    app_state: AppState,
    test_x: f32,
    test_y: f32,
    direction_x: f32,
    direction_y: f32,
    rotation_angle: f64,
}

#[derive(PartialEq, Eq)]
enum AppState {
    Active,
    Quit,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("ハローワールド", 800, 600)
            .position_centered()
            .opengl()
            .build()?;

        let canvas = window.clone().into_canvas().build()?;

        let app_state = AppState::Active;

        Ok(Self {
            sdl_context,
            canvas,
            app_state,
            test_x: 0.0,
            test_y: 0.0,
            direction_x: 1.0,
            direction_y: 1.0,
            rotation_angle: 0.0,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;

        let ttf_context = Rc::new(ttf::init().map_err(|e| e.to_string())?);
        let mut font = ttf_context.load_font("assets/Fonts/VeraMono/VeraMono.ttf", 200)?;

        while self.app_state == AppState::Active {
            i = (i + 1) % 255;

            self.handle_events(&mut event_pump);

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            self.draw_green_border()?;
            self.display_text(i, &mut font)?;
            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.app_state = AppState::Quit,

                _ => {}
            }
        }
    }

    fn draw_green_border(&mut self) -> Result<(), Box<dyn Error>> {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        let rect = Rect::new(
            0,
            0,
            self.canvas.viewport().width(),
            self.canvas.viewport().height(),
        );
        self.canvas.draw_rect(rect)?;
        Ok(())
    }

    fn display_text(&mut self, num: u8, font: &mut Font) -> Result<(), Box<dyn Error>> {
        let hue = (num as f32 / 255.0) * 360.0;
        let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);

        let surface = font.render("You're Gay!").blended(Color::RGB(r, g, b))?;

        let texture_creator = self.canvas.texture_creator();

        let texture = texture_creator.create_texture_from_surface(&surface)?;

        let view_width = self.canvas.viewport().width();
        let view_height = self.canvas.viewport().height();
        let text_width = view_width / 3;
        let text_height = self.canvas.viewport().height() / 6;

        //Update position
        self.test_x += self.direction_x * 5.0;
        self.test_y += self.direction_y * 5.0;

        //rotate
        self.rotation_angle = (self.rotation_angle + 2.0) % 360.0;

        if self.test_x <= 0.0 || self.test_x + text_width as f32 >= view_width as f32 {
            self.direction_x *= -1.0;
        }

        if self.test_y <= 0.0 || self.test_y + text_height as f32 >= view_height as f32 {
            self.direction_y *= -1.0;
        }

        let text_rect = Rect::new(
            (self.test_x) as i32,
            (self.test_y) as i32,
            text_width,
            text_height,
        );

        self.canvas.copy_ex(
            &texture,
            None,
            Some(text_rect),
            self.rotation_angle,
            None,
            false,
            false,
        )?;

        Ok(())
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - (h / 60.0) % 2.0 - 1.0).abs();

    let m = v - c;

    let (r, g, b) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (x, 0.0, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
