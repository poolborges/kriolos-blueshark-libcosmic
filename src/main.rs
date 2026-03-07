use cosmic::app::{Core};
use cosmic::iced::{Alignment, Length};
use cosmic::app::Task; 
use cosmic::widget::{self, column, row, scrollable, text, text_input, container};
use cosmic::{Application, Element};

struct BlueShark {
    core: Core,
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
    const APP_ID: &'static str = "com.github.kriolos.BlueShark";

    // Inicialização obrigatória da nova API
    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = Self {
            core,
            input_value: String::new(),
            messages: vec!["Olá! Eu sou o Blue Shark. Como posso ajudar?".to_string()],
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::InputChanged(val) => {
                self.input_value = val;
                Task::none()
            }
            Message::SendMessage => {
                if !self.input_value.is_empty() {
                    self.messages.push(format!("Tu: {}", self.input_value));
                    self.input_value.clear();
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Blue Shark AI").size(24);

        // Histórico de mensagens
        let mut chat_column = column().spacing(10).width(Length::Fill);
        for m in &self.messages {
            chat_column = chat_column.push(text(m));
        }
        let chat_history = scrollable(chat_column).height(Length::Fill);

        // Área de entrada corrigida
        let input_box = row()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(
                text_input("Escreve a tua mensagem...", &self.input_value)
                    .on_input(Message::InputChanged)
                    .on_submit(|_| Message::SendMessage)
                    .width(Length::Fill)
            )
            .push(
                widget::button::suggested("Enviar")
                    .on_press(Message::SendMessage)
            );

        // Layout final
        container(
            column()
                .spacing(20)
                .push(title)
                .push(chat_history)
                .push(input_box)
        )
        .padding(20)
        .into()
    }
}

fn main() -> cosmic::iced::Result {

    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );
    // Nota: cosmic::run é a forma recomendada na versão estável atual
    cosmic::app::run::<BlueShark>(settings, ())
}
