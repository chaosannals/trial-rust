//use count::Counter;
use iced::{Length, Column, Element, Sandbox, Settings};

//mod count;

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
    }

    fn title(&self) -> String {
        String::from("Trial")
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Units(200))
            .height(Length::Units(100))
            //.push(Counter::new())
            .into()
    }

    fn update(&mut self, message: Message) {
    }
}

fn main() -> iced::Result {
    Trial::run(Settings::default())
}
