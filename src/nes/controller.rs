pub enum ControllerButton {
    // Responses are returned from the controller in this order
    ButtonA,
    ButtonB,
    ButtonSelect,
    ButtonStart,
    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
    Undefined,
}

pub struct Controller {
    is_strobbing: bool,
    next_button: ControllerButton,

    button_a: bool,
    button_b: bool,
    button_select: bool,
    button_start: bool,
    button_up: bool,
    button_down: bool,
    button_left: bool,
    button_right: bool,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            is_strobbing: false,
            next_button: ControllerButton::ButtonA,

            button_a: false,
            button_b: false,
            button_select: false,
            button_start: false,
            button_up: false,
            button_down: false,
            button_left: false,
            button_right: false,
        }
    }

    pub fn write(&mut self, word: u8) {
        self.is_strobbing = (word & 0x1) > 0;
        if self.is_strobbing {
            // Should we check if we're not already on ButtonA? Does it matter?
            self.next_button = ControllerButton::ButtonA;
        }
    }

    #[inline]
    fn set_button_state(&mut self, _button: ControllerButton, state: bool) {
        match self.next_button {
            ControllerButton::ButtonA => {
                self.button_a = state;
            }
            ControllerButton::ButtonB => {
                self.button_b = state;
            }
            ControllerButton::ButtonUp => {
                self.button_up = state;
            }
            ControllerButton::ButtonDown => {
                self.button_down = state;
            }
            ControllerButton::ButtonLeft => {
                self.button_left = state;
            }
            ControllerButton::ButtonRight => {
                self.button_right = state;
            }
            ControllerButton::ButtonStart => {
                self.button_start = state;
            }
            ControllerButton::ButtonSelect => {
                self.button_select = state;
            }
            ControllerButton::Undefined => {
                panic!("attempted to set button Undefined");
            }
        }
    }

    pub fn push_button(&mut self, button: ControllerButton) {
        self.set_button_state(button, true);
    }

    pub fn release_button(&mut self, button: ControllerButton) {
        self.set_button_state(button, false);
    }

    pub fn read(&mut self) -> bool {
        match self.next_button {
            ControllerButton::ButtonA => {
                self.next_button = ControllerButton::ButtonB;
                self.button_a
            }
            ControllerButton::ButtonB => {
                self.next_button = ControllerButton::ButtonSelect;
                self.button_b
            }
            ControllerButton::ButtonSelect => {
                self.next_button = ControllerButton::ButtonStart;
                self.button_select
            }
            ControllerButton::ButtonStart => {
                self.next_button = ControllerButton::ButtonUp;
                self.button_start
            }
            ControllerButton::ButtonUp => {
                self.next_button = ControllerButton::ButtonDown;
                self.button_up
            }
            ControllerButton::ButtonDown => {
                self.next_button = ControllerButton::ButtonLeft;
                self.button_down
            }
            ControllerButton::ButtonLeft => {
                self.next_button = ControllerButton::ButtonRight;
                self.button_left
            }
            ControllerButton::ButtonRight => {
                self.next_button = ControllerButton::Undefined;
                self.button_right
            }
            ControllerButton::Undefined => {
                // We've already read the states of all 8 buttons, now we just
                // return high regardless until the game strobes again
                true
            }
        }
    }
}
