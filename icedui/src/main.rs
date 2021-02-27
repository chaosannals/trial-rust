use iced::{Length, Column, Element, Sandbox, Settings};

#[derive(Debug, Clone)]
struct Message {
}

#[derive(Default)]
struct Trial {
    
}

impl Sandbox for Trial {
    type Message = Message;

    fn new() -> Self {
        Self::default()
            .width(Length::Units(200))
            .height(Length::Units(100))
    }

    fn title(&self) -> String {
        String::from("Trial")
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Units(200))
            .height(Length::Units(100))
            .into()
    }

    fn update(&mut self, message: Message) {
    }
}

fn main() -> iced::Result {
    Trial::run(Settings::default())
}
