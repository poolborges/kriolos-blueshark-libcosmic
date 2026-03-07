use cosmic::app::{Core, Task, Settings};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, column, row, scrollable, text, text_input, container, Space};
use cosmic::{Application, Element, Action};

struct BlueShark {
    core: Core,
    input_value: String,
    messages: Vec<String>,
    is_loading: bool, 
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
        messages: vec!["🦈 Olá! Sou o Blue Shark. Como posso ajudar?".into()],
        is_loading: false, // Inicializa como falso
    };
    (app, Task::none())
}

        fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::InputChanged(val) => {
                if !self.is_loading { self.input_value = val; }
                Task::none()
            }
            Message::SendMessage => {
                if !self.input_value.is_empty() && !self.is_loading {
                    let user_text = self.input_value.clone();
                    self.messages.push(format!("Tu: {}", user_text));
                    self.input_value.clear();
                    self.is_loading = true; // Bloqueia a interface

                    // Conexão real com Ollama
                    return Task::perform(
                        async move {
                            let client = reqwest::Client::new();
                            let res = client.post("http://localhost:11434/api/generate")
                                .json(&serde_json::json!({
                                    "model": "granite-code:3b",//"llama3", 
                                    "prompt": user_text,
                                    "stream": false
                                }))
                                .send()
                                .await;

                            match res {
                                Ok(response) => {
                                    let json: serde_json::Value = response.json().await.unwrap_or_default();
                                    let ai_text = json["response"].as_str().unwrap_or("Sem resposta").to_string();
                                    format!("Blue Shark: {}", ai_text)
                                }
                                Err(_) => "Blue Shark: Erro! O Ollama está ativo?".into(),
                            }
                        },

                        |res| Action::from(Message::AiResponseReceived(res)),
                    );
                }
                Task::none()
            }
            Message::AiResponseReceived(response) => {
                self.messages.push(response);
                self.is_loading = false; // Desbloqueia a interface
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
            let bubble = container(text(m)).padding(12);

            let row_wrapper = if is_user {
                row().push(Space::new().width(Length::Fill)).push(bubble)
            } else {
                row().push(bubble).push(Space::new().width(Length::Fill))
            };
            chat_column = chat_column.push(row_wrapper);
        }

        let chat_history = scrollable(chat_column).height(Length::Fill);

        // Criamos o botão de forma condicional
        let mut send_button = widget::button::suggested("Enviar");
        
        if !self.is_loading && !self.input_value.is_empty() {
            send_button = send_button.on_press(Message::SendMessage);
        }

        let input_box = row()
            .spacing(12)
            .align_y(Alignment::Center)
            .push(
                text_input(
                    if self.is_loading { "O Blue Shark está a pensar..." } else { "Pergunta algo..." }, 
                    &self.input_value
                )
                .on_input(Message::InputChanged)
                .on_submit(|_| Message::SendMessage)
                .width(Length::Fill)
            )
            .push(send_button);

        // Se estiver a carregar, podemos adicionar uma barra de progresso indeterminada
        let mut content = column().spacing(20).push(chat_history).push(input_box);
        
        if self.is_loading {
            content = content.push(widget::progress_bar(0.0..=100.0, 50.0)); // Barra estática ou animada
        }

        column()
            .push(header)
            .push(
                container(content)
                    .padding(20)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .into()
    }
}

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<BlueShark>(Settings::default(), ())
}
