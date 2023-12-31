use regex::Regex;
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_subsystem = sdl2::image::init(InitFlag::all()).unwrap();
    let window = video_subsystem.window("Window", 800, 600).build().unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut sprite = build_sprite(&texture_creator, 400, 400);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    sprite = build_sprite(
                        &texture_creator,
                        sprite.rect.width() + 10,
                        sprite.rect.height() + 10,
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    sprite = build_sprite(
                        &texture_creator,
                        sprite.rect.width() - 10,
                        sprite.rect.height() - 10,
                    );
                }
                _ => {}
            }
        }
        canvas.clear();
        canvas
            .copy(&sprite.texture, None, Some(sprite.rect))
            .unwrap();
        canvas.present();
    }
}

struct Sprite<'a> {
    texture: Texture<'a>,
    rect: Rect,
}

fn build_sprite(texture_creator: &TextureCreator<WindowContext>, h: u32, w: u32) -> Sprite {
    // TODO load this string from a file...
    let svg = "<svg height='400' width='400' viewBox='0 0 400 400'><circle cx='200' cy='200' r='160' stroke='white' stroke-width='4' fill='black'/></svg>";

    // TODO handle errors from these patterns and check whether these could be static? ... also fix to match only to svg element.
    let w_pattern = Regex::new(r"(<svg.* width=')(?:[0-9]+)('.*)").unwrap(); // TODO error handling
    let h_pattern = Regex::new(r"(<svg.* height=')(?:[0-9]+)('.*)").unwrap(); // TODO error handling

    let replace_w = String::from("${1}") + w.to_string().as_str() + "${2}";
    let replace_h = String::from("${1}") + w.to_string().as_str() + "${2}";

    // TODO Check how to handle these 'Cow' values.
    let mut modified_svg = w_pattern.replacen(svg, 1, replace_w).to_string();
    modified_svg = h_pattern.replacen(&modified_svg, 1, replace_h).to_string();

    Sprite {
        texture: texture_creator
            .load_texture_bytes(modified_svg.as_bytes())
            .unwrap(),
        rect: Rect::new(0, 0, h, w),
    }
}
