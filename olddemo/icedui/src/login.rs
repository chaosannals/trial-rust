use iced::{text_input, Column, TextInput, Text};

#[derive(Default)]
pub struct LoginPage {
    value: String,
    acount_state: text_input::State,
    password_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum LoginEvent {
    InputChanged(String),
}

impl LoginPage {
    pub fn new() -> LoginPage {
        LoginPage {
            value: "".to_string(),
            acount_state: text_input::State::new(),
            password_state: text_input::State::new(),
        }
    }

    pub fn view(&self) -> Column<LoginEvent> {
        Column::new()
            .push(
                TextInput::new(
                    &mut self.acount_state,
                    "account",
                    "111",
                    LoginEvent::InputChanged,
                ),
            )
            .push(
                Text::new("login").size(50),
            )
            // .push(
            //     Button::new(&mut self.decrement_button, Text::new("-"))
            //         .on_press(Message::DecrementPressed),
            // )
    }

    pub fn update(&mut self, event: LoginEvent) {
        match event {
            LoginEvent::InputChanged(value) => {
                self.value = value
            }
        }
    }
}