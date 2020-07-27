pub mod input;
pub mod scene;

use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

use sdl2::{
    image::{self, LoadTexture},
    keyboard::{KeyboardState, Keycode, Scancode},
    mouse::{MouseButton, MouseState},
    render::{Texture, TextureCreator, WindowCanvas},
    EventPump, Sdl, VideoSubsystem,
};

use self::scene::Scene;

const CONFIG_FILE_NAME: &str = "config/config.json";

struct Config {
    window_title: String,
    window_size: (u32, u32),
    enable_vsync: bool,
}

pub fn play(initial_scene: Box<dyn Scene>) {
    let config = read_config_file().unwrap();

    let (sdl_context, video_subsystem) = initialise_sdl().unwrap();
    let mut canvas = initialise_canvas(&video_subsystem, &config).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    play_loop(initial_scene, &mut canvas, &mut event_pump);
}

fn read_config_file() -> Result<Config, Box<dyn Error>> {
    let json_config_string = fs::read_to_string(CONFIG_FILE_NAME)?;
    let config_data: serde_json::Value = serde_json::from_str(&json_config_string[..])?;

    Ok(Config {
        window_title: config_data["window"]["title"].as_str().unwrap().to_string(),
        window_size: (
            config_data["window"]["size"]["x"].as_u64().unwrap() as u32,
            config_data["window"]["size"]["y"].as_u64().unwrap() as u32,
        ),
        enable_vsync: config_data["enable-vsync"].as_bool().unwrap(),
    })
}

fn initialise_sdl() -> Result<(Sdl, VideoSubsystem), String> {
    let sdl_context = sdl2::init()?;
    image::init(image::InitFlag::PNG)?;
    let video_subsystem = sdl_context.video()?;

    Ok((sdl_context, video_subsystem))
}

fn initialise_canvas(
    video_subsystem: &VideoSubsystem,
    config: &Config,
) -> Result<WindowCanvas, Box<dyn Error>> {
    let window = video_subsystem
        .window(
            &config.window_title[..],
            config.window_size.0,
            config.window_size.1,
        )
        .position_centered()
        .allow_highdpi()
        .build()?;

    let canvas = if config.enable_vsync {
        window.into_canvas().accelerated().present_vsync().build()?
    } else {
        window.into_canvas().accelerated().build()?
    };

    Ok(canvas)
}

fn play_loop(initial_scene: Box<dyn Scene>, canvas: &mut WindowCanvas, event_pump: &mut EventPump) {
    let texture_creator = canvas.texture_creator();

    let mut scene_queue = VecDeque::<Box<dyn Scene>>::new();
    let mut current_scene = initial_scene;
    let mut textures = create_textures(&texture_creator, &current_scene.on_load(&canvas));
    current_scene.on_late_load(&canvas, &textures);

    let mut ticks_count = Instant::now();
    let mut is_running = true;

    let mut size_before_fullscreen = None;

    let mut previous_keys: Vec<Scancode> = vec![];
    let mut previous_mouse_buttons: Vec<MouseButton> = vec![];
    let mut mouse_y_scroll_amount = 0;

    while is_running {
        let delta_time = calculate_delta_time(&mut ticks_count);

        poll_events(
            &mut current_scene,
            event_pump,
            canvas,
            &mut size_before_fullscreen,
            &mut is_running,
            &mut mouse_y_scroll_amount,
        );

        process_input(
            &mut current_scene,
            (&event_pump.keyboard_state(), &previous_keys),
            (&event_pump.mouse_state(), &previous_mouse_buttons),
            (event_pump.mouse_state().x(), event_pump.mouse_state().y()),
            mouse_y_scroll_amount,
        );

        update(&mut current_scene, delta_time, &mut scene_queue, &canvas);
        late_update(&mut current_scene, delta_time, &mut scene_queue, &canvas);
        draw(&mut current_scene, canvas, &textures);

        previous_keys = input::update_key_state(&event_pump.keyboard_state());
        previous_mouse_buttons = input::update_mouse_button_state(&event_pump.mouse_state());
        mouse_y_scroll_amount = 0;

        if let Some(ref new_scene_textures) = update_scene_queue(
            &mut current_scene,
            &mut scene_queue,
            &canvas,
            &mut is_running,
        ) {
            textures = create_textures(&texture_creator, new_scene_textures);
            current_scene.on_late_load(&canvas, &textures);
        }
    }
}

fn calculate_delta_time(ticks_count: &mut Instant) -> f32 {
    const MICROSECONDS_PER_SECOND: f32 = 1e6;

    let delta_time = (Instant::now() - *ticks_count).as_micros() as f32 / MICROSECONDS_PER_SECOND;
    *ticks_count = Instant::now();

    delta_time
}

fn poll_events(
    current_scene: &mut Box<dyn Scene>,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    size_before_fullscreen: &mut Option<(u32, u32)>,
    is_running: &mut bool,
    mouse_y_scroll_amount: &mut i32,
) {
    use sdl2::event::Event::*;
    use sdl2::event::WindowEvent::*;

    for event in event_pump.poll_iter() {
        match event {
            Quit { .. }
            | Window {
                win_event: Close, ..
            } => {
                *is_running = false;
            }
            KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                toggle_fullscreen(canvas, size_before_fullscreen);
            }
            MouseWheel { y, .. } => {
                *mouse_y_scroll_amount = y;
            }
            _ => {}
        }

        current_scene.poll_event(event);
    }
}

fn process_input(
    current_scene: &mut Box<dyn Scene>,
    key_states: (&KeyboardState, &[Scancode]),
    mouse_states: (&MouseState, &[MouseButton]),
    mouse_coordinates: (i32, i32),
    mouse_y_scroll_amount: i32,
) {
    let (current_keys, previous_keys) = key_states;
    let (current_mouse_buttons, previous_mouse_buttons) = mouse_states;
    let (mouse_x, mouse_y) = mouse_coordinates;

    let input_state = input::InputState::new(
        current_keys,
        previous_keys,
        current_mouse_buttons,
        previous_mouse_buttons,
        (mouse_x, mouse_y),
        mouse_y_scroll_amount,
    );

    current_scene.process_input(&input_state);
}

fn update(
    current_scene: &mut Box<dyn Scene>,
    delta_time: f32,
    scene_queue: &mut VecDeque<Box<dyn Scene>>,
    canvas: &WindowCanvas,
) {
    current_scene.update(delta_time, scene_queue, canvas);
}

fn late_update(
    current_scene: &mut Box<dyn Scene>,
    delta_time: f32,
    scene_queue: &mut VecDeque<Box<dyn Scene>>,
    canvas: &WindowCanvas,
) {
    current_scene.late_update(delta_time, scene_queue, canvas);
}

fn draw(current_scene: &mut Box<dyn Scene>, canvas: &mut WindowCanvas, textures: &[Texture]) {
    current_scene.draw(canvas, textures);

    canvas.present();
}

fn update_scene_queue(
    current_scene: &mut Box<dyn Scene>,
    scene_queue: &mut VecDeque<Box<dyn Scene>>,
    canvas: &WindowCanvas,
    is_running: &mut bool,
) -> Option<Vec<String>> {
    if current_scene.is_done() {
        current_scene.on_unload();

        if !scene_queue.is_empty() {
            *current_scene = scene_queue.pop_front().unwrap();
            Some(current_scene.on_load(canvas))
        } else {
            *is_running = false;

            None
        }
    } else {
        None
    }
}

fn create_textures<'a>(
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    texture_filepaths: &[String],
) -> Vec<Texture<'a>> {
    let mut textures = vec![];

    for texture_path in texture_filepaths {
        textures.push(texture_creator.load_texture(texture_path).unwrap());
    }

    textures
}

fn toggle_fullscreen(canvas: &mut WindowCanvas, size_before_fullscreen: &mut Option<(u32, u32)>) {
    use sdl2::video::FullscreenType;

    let current_viewport_size = (canvas.viewport().width(), canvas.viewport().height());
    let window = canvas.window_mut();

    match window.fullscreen_state() {
        FullscreenType::True | FullscreenType::Desktop => {
            window
                .set_size(
                    size_before_fullscreen.unwrap().0,
                    size_before_fullscreen.unwrap().1,
                )
                .unwrap();

            window.set_fullscreen(FullscreenType::Off).unwrap();
            window.set_bordered(true);

            *size_before_fullscreen = None;
        }
        FullscreenType::Off => {
            *size_before_fullscreen = Some(current_viewport_size);
            window.set_fullscreen(FullscreenType::True).unwrap();
            window.set_bordered(false);

            let display_mode = window.display_mode().unwrap();

            window
                .set_size(display_mode.w as u32, display_mode.h as u32)
                .unwrap();
        }
    }
}
