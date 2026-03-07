use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, column, row, scrollable, text, text_input, button, container};
use cosmic::{Application, ApplicationExt, Element};

struct BlueShark {
    input_value: String,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendMessage,
}

impl Application for BlueShark {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = cosmic::theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, cosmic::Command<Self::Message>) {
        (
            Self {
                input_value: String::new(),
                messages: vec!["Olá! Eu sou o Blue Shark. Como posso ajudar Cabo Verde hoje?".to_string()],
            },
            cosmic::Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> cosmic::Command<Self::Message> {
        match message {
            Message::InputChanged(val) => {
                self.input_value = val;
                cosmic::Command::none()
            }
            Message::SendMessage => {
                if !self.input_value.is_empty() {
                    self.messages.push(format!("Tu: {}", self.input_value));
                    self.input_value.clear();
                    // Aqui chamaremos a lógica da IA depois
                }
                cosmic::Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Blue Shark AI").size(24);

        let chat_history = scrollable(
            column(self.messages.iter().map(|m| text(m).into()).collect())
                .spacing(10)
                .width(Length::Fill)
        ).height(Length::Fill);

        let input_box = row![
            text_input("Escreve a tua mensagem...", &self.input_value)
                .on_input(Message::InputChanged)
                .on_submit(Message::SendMessage),
            button("Enviar").on_press(Message::SendMessage)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(column![title, chat_history, input_box].spacing(20))
            .padding(20)
            .into()
    }
}

fn main() -> cosmic::iced::Result {
    BlueShark::run_window(cosmic::Settings::default())
}
