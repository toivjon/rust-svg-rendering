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

    let mut sprite = Sprite {
        rect: Rect::new(0, 0, 400, 400),
        texture: svg_circle(&texture_creator, 400, 400),
    };

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
                    sprite.rect.set_width(sprite.rect.width() + 10);
                    sprite.rect.set_height(sprite.rect.height() + 10);
                    sprite.texture =
                        svg_circle(&texture_creator, sprite.rect.height(), sprite.rect.width());
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    sprite.rect.set_width(sprite.rect.width() - 10);
                    sprite.rect.set_height(sprite.rect.height() - 10);
                    sprite.texture =
                        svg_circle(&texture_creator, sprite.rect.height(), sprite.rect.width());
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

fn svg_circle(texture_creator: &TextureCreator<WindowContext>, h: u32, w: u32) -> Texture {
    let svg = format!("<svg height='{}' width='{}' viewBox='0 0 400 400'><circle cx='200' cy='200' r='160' stroke='white' stroke-width='4' fill='black'/></svg>", h, w);
    texture_creator.load_texture_bytes(svg.as_bytes()).unwrap()
}
