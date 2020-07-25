pub mod input;
pub mod scene;

use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    mouse::{MouseButton, MouseState},
    render::WindowCanvas,
    EventPump, Sdl, VideoSubsystem,
};

use self::scene::Scene;

const CONFIG_FILE_NAME: &str = "config/config.json";

struct Config {
    window_title: String,
    window_size: (u32, u32),
}

pub fn play(initial_scene: Box<dyn Scene>) {
    let config = read_config_file().unwrap();

    let (sdl_context, video_subsystem) = initialise_sdl().unwrap();
    let mut canvas = initialise_canvas(&video_subsystem, &config).unwrap();

    let texture_creator = canvas.texture_creator();
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
    })
}

fn initialise_sdl() -> Result<(Sdl, VideoSubsystem), String> {
    let sdl_context = sdl2::init()?;
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

    let canvas = window.into_canvas().accelerated().present_vsync().build()?;

    Ok(canvas)
}

fn play_loop(initial_scene: Box<dyn Scene>, canvas: &mut WindowCanvas, event_pump: &mut EventPump) {
    let mut scene_queue = VecDeque::<Box<dyn Scene>>::new();
    let mut current_scene = initial_scene;
    current_scene.on_load();

    let mut ticks_count = Instant::now();
    let mut is_running = true;

    let mut previous_keys: Vec<Scancode> = vec![];
    let mut previous_mouse_buttons: Vec<MouseButton> = vec![];
    let mut mouse_y_scroll_amount = 0;

    while is_running {
        let delta_time = calculate_delta_time(&mut ticks_count);
        poll_events(event_pump, &mut is_running, &mut mouse_y_scroll_amount);

        process_input(
            &mut current_scene,
            (&event_pump.keyboard_state(), &previous_keys),
            (&event_pump.mouse_state(), &previous_mouse_buttons),
            (event_pump.mouse_state().x(), event_pump.mouse_state().y()),
            mouse_y_scroll_amount,
            &mut is_running,
        );

        update(&mut current_scene, delta_time, &mut scene_queue);
        draw(&mut current_scene, canvas);

        previous_keys = input::update_key_state(&event_pump.keyboard_state());
        previous_mouse_buttons = input::update_mouse_button_state(&event_pump.mouse_state());
        mouse_y_scroll_amount = 0;

        update_scene_queue(&mut current_scene, &mut scene_queue, &mut is_running);
    }
}

fn calculate_delta_time(ticks_count: &mut Instant) -> f32 {
    const MICROSECONDS_PER_SECOND: f32 = 1_000_000.0;

    let delta_time = (Instant::now() - *ticks_count).as_micros() as f32 / MICROSECONDS_PER_SECOND;
    *ticks_count = Instant::now();

    delta_time
}

fn poll_events(event_pump: &mut EventPump, is_running: &mut bool, mouse_y_scroll_amount: &mut i32) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                *is_running = false;
            }
            Event::MouseWheel { y, .. } => {
                *mouse_y_scroll_amount = y;
            }
            _ => {}
        }
    }
}

fn process_input(
    current_scene: &mut Box<dyn Scene>,
    key_states: (&KeyboardState, &[Scancode]),
    mouse_states: (&MouseState, &[MouseButton]),
    mouse_coordinates: (i32, i32),
    mouse_y_scroll_amount: i32,
    is_running: &mut bool,
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

    if input_state.is_key_down(Scancode::Escape) {
        *is_running = false;
    }

    current_scene.process_input(&input_state);
}

fn update(
    current_scene: &mut Box<dyn Scene>,
    delta_time: f32,
    scene_queue: &mut VecDeque<Box<dyn Scene>>,
) {
    current_scene.update(delta_time, scene_queue);
}

fn draw(current_scene: &mut Box<dyn Scene>, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(current_scene.background_colour());
    canvas.clear();

    current_scene.draw(canvas);

    canvas.present();
}

fn update_scene_queue(
    current_scene: &mut Box<dyn Scene>,
    scene_queue: &mut VecDeque<Box<dyn Scene>>,
    is_running: &mut bool,
) {
    if current_scene.is_done() {
        current_scene.on_unload();

        if !scene_queue.is_empty() {
            *current_scene = scene_queue.pop_front().unwrap();
            current_scene.on_load();
        } else {
            *is_running = false;
        }
    }
}
