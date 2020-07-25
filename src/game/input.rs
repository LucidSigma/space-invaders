use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::{MouseButton, MouseState};

pub struct InputState<'a> {
    current_keys: &'a KeyboardState<'a>,
    previous_keys: &'a [Scancode],

    current_mouse_buttons: &'a MouseState,
    previous_mouse_buttons: &'a [MouseButton],

    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_y_scroll: i32,
}

impl<'a> InputState<'a> {
    pub fn new(
        current_keys: &'a KeyboardState,
        previous_keys: &'a [Scancode],
        current_mouse_buttons: &'a MouseState,
        previous_mouse_buttons: &'a [MouseButton],
        mouse_coordinates: (i32, i32),
        mouse_y_scroll: i32,
    ) -> InputState<'a> {
        InputState {
            current_keys,
            previous_keys,
            current_mouse_buttons,
            previous_mouse_buttons,
            mouse_x: mouse_coordinates.0,
            mouse_y: mouse_coordinates.1,
            mouse_y_scroll,
        }
    }

    #[allow(dead_code)]
    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.current_keys.is_scancode_pressed(scancode)
    }

    #[allow(dead_code)]
    pub fn is_any_key_pressed(&self, scancodes: &[Scancode]) -> bool {
        scancodes
            .iter()
            .any(|scancode| self.is_key_pressed(*scancode))
    }

    #[allow(dead_code)]
    pub fn are_all_keys_pressed(&self, scancodes: &[Scancode]) -> bool {
        scancodes
            .iter()
            .all(|scancode| self.is_key_pressed(*scancode))
    }

    #[allow(dead_code)]
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.current_keys.is_scancode_pressed(scancode)
            && !self.previous_keys.iter().any(|code| code == &scancode)
    }

    #[allow(dead_code)]
    pub fn is_any_key_down(&self, scancodes: &[Scancode]) -> bool {
        scancodes.iter().any(|scancode| self.is_key_down(*scancode))
    }

    #[allow(dead_code)]
    pub fn are_all_keys_down(&self, scancodes: &[Scancode]) -> bool {
        scancodes.iter().all(|scancode| self.is_key_down(*scancode))
    }

    #[allow(dead_code)]
    pub fn is_key_up(&self, scancode: Scancode) -> bool {
        !self.current_keys.is_scancode_pressed(scancode)
            && self.previous_keys.iter().any(|code| code == &scancode)
    }

    #[allow(dead_code)]
    pub fn is_any_key_up(&self, scancodes: &[Scancode]) -> bool {
        scancodes.iter().any(|scancode| self.is_key_up(*scancode))
    }

    #[allow(dead_code)]
    pub fn are_all_keys_up(&self, scancodes: &[Scancode]) -> bool {
        scancodes.iter().all(|scancode| self.is_key_up(*scancode))
    }

    #[allow(dead_code)]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.current_mouse_buttons.is_mouse_button_pressed(button)
    }

    #[allow(dead_code)]
    pub fn is_any_mouse_button_pressed(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .any(|button| self.is_mouse_button_pressed(*button))
    }

    #[allow(dead_code)]
    pub fn are_all_mouse_buttons_pressed(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .all(|button| self.is_mouse_button_pressed(*button))
    }

    #[allow(dead_code)]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.current_mouse_buttons.is_mouse_button_pressed(button)
            && !&self
                .previous_mouse_buttons
                .iter()
                .any(|previous_button| previous_button == &button)
    }

    #[allow(dead_code)]
    pub fn is_any_mouse_button_down(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .any(|button| self.is_mouse_button_down(*button))
    }

    #[allow(dead_code)]
    pub fn are_all_mouse_buttons_down(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .all(|button| self.is_mouse_button_down(*button))
    }

    #[allow(dead_code)]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        !self.current_mouse_buttons.is_mouse_button_pressed(button)
            && self
                .previous_mouse_buttons
                .iter()
                .any(|previous_button| previous_button == &button)
    }

    #[allow(dead_code)]
    pub fn is_any_mouse_button_up(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .any(|button| self.is_mouse_button_up(*button))
    }

    #[allow(dead_code)]
    pub fn are_all_mouse_buttons_up(&self, buttons: &[MouseButton]) -> bool {
        buttons
            .iter()
            .all(|button| self.is_mouse_button_up(*button))
    }
}

pub fn update_key_state(current_keys: &KeyboardState) -> Vec<Scancode> {
    let mut previous_keys = vec![];
    previous_keys.reserve_exact(16);

    for scancode in current_keys.pressed_scancodes() {
        previous_keys.push(scancode);
    }

    previous_keys
}

pub fn update_mouse_button_state(current_buttons: &MouseState) -> Vec<MouseButton> {
    let mut previous_buttons = vec![];
    previous_buttons.reserve_exact(5);

    for button in current_buttons.pressed_mouse_buttons() {
        previous_buttons.push(button);
    }

    previous_buttons
}
