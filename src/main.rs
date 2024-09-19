use iced::widget::{button, column, text};
use iced::Element;

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    column![
        text(counter.value).size(20),
        button("Increment").on_press(Message::Increment),
        button("Decrement").on_press(Message::Decrement),
    ]
        .spacing(10)
        .into()
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}

#[derive(Default)]
struct Counter {
    value: u64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}