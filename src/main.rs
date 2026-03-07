use cosmic::app::{Core, Task, Settings};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, icon, column, row, scrollable, text, text_input, container, Space};
use cosmic::{Application, Element, Action};

#[derive(Debug, Clone)]
struct Chat {
    title: String,
    messages: Vec<String>,
}

struct BlueShark {
    core: Core,
    chats: Vec<Chat>,
    current_chat_idx: usize,
    input_value: String,
    is_loading: bool,
    selected_model: String,
    available_models: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendMessage,
    AiResponseReceived(String),
    SelectChat(usize),
    NewChat,
    ModelSelected(usize),
    OpenModelManager,
    AttachFile,
    MicrophoneAction,
}

impl Application for BlueShark {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.kriolos.BlueShark";

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let initial_chat = Chat {
            title: "Chat Inicial".into(),
            messages: vec!["🦈 Olá! Sou o Blue Shark. Como posso ajudar?".into()],
        };

        let app = Self {
            core,
            chats: vec![initial_chat],
            current_chat_idx: 0,
            input_value: String::new(),
            is_loading: false,
            selected_model: "granite-code:3b".into(),
            available_models: vec!["granite-code:3b".into(), "granite-code:8b".into()],
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
                    self.chats[self.current_chat_idx].messages.push(format!("Tu: {}", user_text));
                    self.input_value.clear();
                    self.is_loading = true;

                    let model = self.selected_model.clone();
                    return Task::perform(
                        async move {
                            let client = reqwest::Client::new();
                            let res = client.post("http://localhost:11434/api/generate")
                                .json(&serde_json::json!({
                                    "model": model,
                                    "prompt": user_text,
                                    "stream": false
                                }))
                                .send().await;

                            match res {
                                Ok(resp) => {
                                    let json: serde_json::Value = resp.json().await.unwrap_or_default();
                                    let txt = json["response"].as_str().unwrap_or("Sem resposta").to_string();
                                    format!("Blue Shark: {}", txt)
                                }
                                Err(_) => "Erro: Ollama offline?".into(),
                            }
                        },

                        |res| Action::from(Message::AiResponseReceived(res)),
                    );
                }
                Task::none()
            }
            Message::AiResponseReceived(resp) => {
                self.chats[self.current_chat_idx].messages.push(resp);
                self.is_loading = false;
                Task::none()
            }
            Message::SelectChat(idx) => {
                self.current_chat_idx = idx;
                Task::none()
            }
            Message::NewChat => {
                let new_idx = self.chats.len();
                self.chats.push(Chat {
                    title: format!("Chat {}", new_idx + 1),
                    messages: vec!["🦈 Novo chat iniciado. Como posso ajudar?".into()],
                });
                self.current_chat_idx = new_idx;
                Task::none()
            }
            Message::ModelSelected(idx) => {
                if let Some(model_name) = self.available_models.get(idx) {
                    self.selected_model = model_name.clone();
                }
                Task::none()
            }
            _ => Task::none(), // Outras ações (Manager, Attach, Micro) para futura implementação
        }
    }

        fn view(&self) -> Element<'_, Self::Message> {
        // --- SIDEBAR ---
        let mut chat_list = column().spacing(8).width(Length::Fixed(220.0));
        chat_list = chat_list.push(
            widget::button::suggested("+ Novo Chat")
                .on_press(Message::NewChat)
                .width(Length::Fill)
        );
        
        for (i, chat) in self.chats.iter().enumerate() {
            let is_selected = i == self.current_chat_idx;
            let mut btn = widget::button::standard(&chat.title).width(Length::Fill);
            if !is_selected { btn = btn.on_press(Message::SelectChat(i)); }
            chat_list = chat_list.push(btn);
        }

        // CORREÇÃO: Estilo da Sidebar usando o caminho correto
        let sidebar = container(chat_list)
            .padding(10)
            .height(Length::Fill)
            .class(cosmic::theme::Container::Card);

        // --- HISTÓRICO DE CHAT ---
        let mut chat_column = column().spacing(12).width(Length::Fill);
        let current_chat = &self.chats[self.current_chat_idx];
        
        for m in &current_chat.messages {
            let is_user = m.starts_with("Tu:");

            let display_text = if is_user { 
                m.trim_start_matches("Tu: ").to_string() 
            } else { 
                m.trim_start_matches("Blue Shark: ").to_string() 
            };
            
            // CORREÇÃO: Estilo das bolhas de chat
            let bubble = container(text(display_text))
                .padding(10)
                .class(if is_user {
                    cosmic::theme::Container::Secondary
                } else {
                    cosmic::theme::Container::Primary
                });

            let row_wrapper = if is_user {
                row().push(Space::new().width(Length::Fill)).push(bubble)
            } else {
                row().push(bubble).push(Space::new().width(Length::Fill))
            };
            chat_column = chat_column.push(row_wrapper);
        }
        let chat_history = scrollable(chat_column).height(Length::Fill);

        // --- BARRA INFERIOR (Modelos e Ícones) ---
        /*
        let btn_com_ambos = widget::button::standard(
            row().push(
                icon::from_name("settings-symbolic"))
                .push(
                text("Definições"))
            .spacing(8)
        ).on_press(Message::OpenModelManager);
        */
        // Botão apenas com ícone
        /*
        let settings_btn = widget::button::icon(icon::from_name("settings-symbolic"))
            .on_press(Message::OpenModelManager);

        let settings_btn = widget::button::icon(icon::from_name("preferences-system-symbolic")
        .symbolic(true) /*Garante que a cor se ajusta ao tema Dark/Light*/)
        .on_press(Message::OpenModelManager);

    let attach_btn = widget::button::icon(icon::from_name("mail-attachment-symbolic"))
            .on_press(Message::AttachFile);

        let mic_btn = widget::button::icon(icon::from_name("audio-input-microphone-symbolic"))
            .on_press(Message::MicrophoneAction);


            //.push(widget::button::standard("⚙️").on_press(Message::OpenModelManager))
    */

        let settings_btn = widget::button::standard("⚙")
            .on_press(Message::OpenModelManager);

        let attach_btn = widget::button::standard("📎")
            .on_press(Message::AttachFile);

        let mic_btn = widget::button::standard("🎤")
            .on_press(Message::MicrophoneAction);

        

        // CORREÇÃO: Encontrar o índice do modelo selecionado
        let selected_idx = self.available_models
        .iter()
        .position(|m| m == &self.selected_model);

        let model_dropdown = widget::dropdown(
            &self.available_models,
            selected_idx, // Agora passamos o Option<usize>
            Message::ModelSelected,
        ).width(Length::Fixed(150.0));

        let icons_row = row().spacing(15).align_y(Alignment::Center)
            .push(settings_btn)
            .push(attach_btn)
            .push(mic_btn)
            .push(Space::new().width(Length::Fill))
            //.push(text(format!("Modelo: {}", self.selected_model)).size(14))
            .push(text("Modelo").size(14))
            .push(model_dropdown);

        let send_btn = widget::button::suggested("Enviar");
        let send_btn = if self.is_loading || self.input_value.is_empty() {
            send_btn // Botão desativado (sem .on_press)
        } else {
            send_btn.on_press(Message::SendMessage)
        };

        let input_row = row().spacing(10).align_y(Alignment::Center)
            .push(text_input(if self.is_loading { "A pensar..." } else { "Pergunta algo..." }, &self.input_value)
                .on_input(Message::InputChanged).on_submit(|_| Message::SendMessage).width(Length::Fill))
            .push(send_btn);

        let bottom_area = column().spacing(10).push(input_row).push(icons_row);

        // --- LAYOUT FINAL ---
        let main_view = container(column().spacing(15).push(chat_history).push(bottom_area))
            .padding(20).width(Length::Fill).height(Length::Fill);

        column()
            //.push(widget::header_bar().title("Blue Shark AI"))
            .push(row().push(sidebar).push(main_view))
            .into()
    }

}

fn main() -> cosmic::iced::Result {
    let settings = Settings::default();
    cosmic::app::run::<BlueShark>(settings, ())
}
