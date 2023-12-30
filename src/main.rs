use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    rwops::RWops,
    sys::{
        image::IMG_LoadSVG_RW, SDL_CreateRenderer, SDL_CreateTextureFromSurface, SDL_Rect,
        SDL_RenderClear, SDL_RenderCopy, SDL_RenderPresent, SDL_Renderer, SDL_Texture,
    },
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_subsystem = sdl2::image::init(InitFlag::all()).unwrap();
    let window = video_subsystem.window("Window", 800, 600).build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = unsafe { SDL_CreateRenderer(window.raw(), 0, 0) };

    let mut width = 400;
    let mut height = 400;
    let mut texture = svg_circle(renderer, width, height);

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
                    texture = svg_circle(renderer, height, width);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    height -= 10;
                    width -= 10;
                    texture = svg_circle(renderer, height, width);
                }
                _ => {}
            }
        }
        let source_rect = SDL_Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        };
        let texture_rect = SDL_Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        };
        unsafe { SDL_RenderClear(renderer) };
        unsafe { SDL_RenderCopy(renderer, texture, &source_rect, &texture_rect) };
        unsafe { SDL_RenderPresent(renderer) };
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn svg_circle(renderer: *mut SDL_Renderer, h: i32, w: i32) -> *mut SDL_Texture {
    let svg = format!("<svg height='{}' width='{}' viewBox='0 0 400 400'><circle cx='200' cy='200' r='160' stroke='white' stroke-width='4' fill='black'/></svg>", h, w);
    let data = RWops::from_bytes(svg.as_bytes()).unwrap();
    let surface = unsafe { IMG_LoadSVG_RW(data.raw()) };
    unsafe { SDL_CreateTextureFromSurface(renderer, surface) }
}
