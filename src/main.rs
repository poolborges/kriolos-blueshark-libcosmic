use cosmic::app::{Core, Task, Settings};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, column, row, scrollable, text, text_input, container, Space};
use cosmic::{Application, Element, Action}; // Importa Action

struct BlueShark {
    core: Core,
    input_value: String,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendMessage,
    AiResponseReceived(String),
}

impl Application for BlueShark {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.kriolos.BlueShark";

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = Self {
            core,
            input_value: String::new(),
            messages: vec!["🦈 Olá! Eu sou o Blue Shark. Como posso ajudar?".to_string()],
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
                    let user_text = self.input_value.clone();
                    self.messages.push(format!("Tu: {}", user_text));
                    self.input_value.clear();

                    // CORREÇÃO: Task::perform espera Action::from(Message)
                    return Task::perform(
                        async move {
                            tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                            format!("Blue Shark: Analisei a tua mensagem sobre '{}'.", user_text)
                        },

                        |res| Action::from(Message::AiResponseReceived(res)),
                    );
                }
                Task::none()
            }
            Message::AiResponseReceived(response) => {
                self.messages.push(response);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = widget::header_bar()
            .title("Blue Shark AI")
            .start(row().push(text("🦈").size(24)))
            .end(row().push(text("v0.1.0").size(12)));

        let mut chat_column = column().spacing(15).width(Length::Fill);
        
        for m in &self.messages {
            let is_user = m.starts_with("Tu:");
            
            // Usamos apenas o container com padding. 
            // O COSMIC aplicará o estilo de fundo padrão automaticamente.
            let bubble = container(text(m))
                .padding(12);

            let row_wrapper = if is_user {
                row()
                    .push(Space::new().width(Length::Fill))
                    .push(bubble)
            } else {
                row()
                    .push(bubble)
                    .push(Space::new().width(Length::Fill))
            };

            chat_column = chat_column.push(row_wrapper);
        }

        let chat_history = scrollable(chat_column).height(Length::Fill);

        let input_box = row()
            .spacing(12)
            .align_y(Alignment::Center)
            .push(
                text_input("Escreve aqui...", &self.input_value)
                    .on_input(Message::InputChanged)
                    .on_submit(|_| Message::SendMessage)
                    .width(Length::Fill)
            )
            .push(widget::button::suggested("Enviar").on_press(Message::SendMessage));

        column()
            .push(header)
            .push(
                container(column().spacing(20).push(chat_history).push(input_box))
                    .padding(20)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .into()
    }
}

fn main() -> cosmic::iced::Result {
    let settings = Settings::default();
    cosmic::app::run::<BlueShark>(settings, ())
}
