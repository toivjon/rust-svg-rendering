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

    let mut width = 400;
    let mut height = 400;
    let mut texture = svg_circle(&texture_creator, width, height);

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
                    height += 10;
                    width += 10;
                    texture = svg_circle(&texture_creator, height, width);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    height -= 10;
                    width -= 10;
                    texture = svg_circle(&texture_creator, height, width);
                }
                _ => {}
            }
        }
        let texture_rect = Rect::new(0, 0, width as u32, height as u32);
        canvas.clear();
        canvas.copy(&texture, None, Some(texture_rect)).unwrap();
        canvas.present();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn svg_circle(texture_creator: &TextureCreator<WindowContext>, h: i32, w: i32) -> Texture {
    let svg = format!("<svg height='{}' width='{}' viewBox='0 0 400 400'><circle cx='200' cy='200' r='160' stroke='white' stroke-width='4' fill='black'/></svg>", h, w);
    texture_creator.load_texture_bytes(svg.as_bytes()).unwrap()
}
