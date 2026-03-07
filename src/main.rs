use std::collections::HashMap;

use cosmic::app::{Core, Task, Settings};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, Space, column, container, icon, menu, row, scrollable, text, text_input};
use cosmic::{Application, Element, Action};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

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
    sidebar_visible: bool, // controlar a visibilidade da sidebar
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendMessage,
    AiResponseReceived(String),
    SelectChat(usize),
    ChatOptionsOpen(usize),
    ChatOptionsDelete(usize),      // Exemplo de ação futura
    NewChat,
    ModelSelected(usize),
    OpenModelManager,
    AttachFile,
    MicrophoneAction,
    ToggleSidebar,
}

impl menu::Action for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::SendMessage,
        }
    }
}

impl BlueShark {
    fn new(core: Core) -> Self {
        let mut key_binds = HashMap::new();
        //key_binds.insert(menu::KeyBind::new("About", "Ctrl+A"), MenuAction::About);

        let initial_chat = Chat {
            title: "Chat Inicial".into(),
            messages: vec!["🦈 Olá! Sou o Blue Shark. Como posso ajudar?".into()],
        };

        Self {
            core,
            //chats: Vec::new(),
            chats: vec![initial_chat],  
            current_chat_idx: 0,
            input_value: String::new(),
            is_loading: false,
            selected_model: "granite-code:3b".into(),
            available_models: vec!["granite-code:3b".into(), "granite-code:8b".into()],
            sidebar_visible: true,
            key_binds,
        }
    }

    fn render_sidebar(&self) -> Element<'_, Message> {
        // Contentor da lista de chats
        let mut chat_list = column()
            .spacing(8)
            .padding(10)
            .width(Length::Fixed(240.0))
            .align_x(Alignment::Start); // Largura padrão para sidebars de chat

        // Botão de Novo Chat com Emoji
        chat_list = chat_list.push(
            widget::button::suggested("➕ Novo Chat")
                .on_press(Message::NewChat)
                .width(Length::Fill)
        );
        
        // Lista de conversas existentes
        let mut scrollable_list = column()
            .spacing(5)
            .width(Length::Fill)
            .align_x(Alignment::Start); // Alinhamento à esquerda para os itens da lista

        for (i, chat) in self.chats.iter().enumerate() {
            let is_selected = i == self.current_chat_idx;

            // 1. Truncagem do título
            let display_title = if chat.title.chars().count() > 18 {
                format!("{}...", chat.title.chars().take(15).collect::<String>())
            } else {
                chat.title.clone()
            };

            // 2. Botão Principal (Ocupa o espaço disponível)
            let main_btn = if is_selected {
                widget::button::suggested(display_title)
            } else {
                widget::button::standard(display_title)
            }
            .width(Length::Fill) // Faz este botão "esticar"
            .on_press(Message::SelectChat(i));

            // 3. Botão de Opções (3 pontos)
            let options_btn = widget::button::standard("⋮")
                .on_press(Message::ChatOptionsOpen(i))
                .padding(5); // Mensagem para abrir menu

            // 4. Juntar ambos numa Row
            let item_row = row()
                .push(main_btn)
                .push(options_btn)
                .spacing(4)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            scrollable_list = scrollable_list.push(item_row);
        }


        // Envolver a lista num scrollable para quando houver muitos chats
        let chat_history = scrollable(scrollable_list)
            .height(Length::Fill)
            .width(Length::Fill);

        chat_list = chat_list.push(chat_history);

        // O Container define o "look" da sidebar (fundo ligeiramente diferente)
        container(chat_list)
            .height(Length::Fill)
            .width(Length::Fixed(240.0)) // Reforça a largura no container final
            .class(cosmic::theme::Container::Card) // Dá o aspeto de painel lateral
            .into()
    }


    fn render_chat_area1(&self) -> Element<'_, Message> {
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
     

        let settings_btn = widget::button::standard("⚙")
            .on_press(Message::OpenModelManager);

        let attach_btn = widget::button::standard("📎")
            .on_press(Message::AttachFile);

        let mic_btn = widget::button::standard("🎤")
            .on_press(Message::MicrophoneAction);

        

        // DROPDOWN DE MODELOS
        let selected_idx = self.available_models
        .iter()
        .position(|m| m == &self.selected_model);

        let model_dropdown = widget::dropdown(
            &self.available_models,
            selected_idx, // Agora passamos o Option<usize>
            Message::ModelSelected,
        ).width(Length::Fixed(150.0));

        // ACTION: ícones e dropdown
        let actions_row = row().spacing(15).align_y(Alignment::Center)
            .push(settings_btn)
            .push(attach_btn)
            .push(mic_btn)
            .push(Space::new().width(Length::Fill))
            //.push(text(format!("Modelo: {}", self.selected_model)).size(14))
            .push(text("Modelo").size(14))
            .push(model_dropdown);

        // --- INPUT E BOTÃO DE ENVIO ---
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

        let bottom_area = column().spacing(10)
            .push(input_row)
            .push(actions_row);

        let main_view = container(column().spacing(15)
        .push(chat_history)
        .push(bottom_area))
            .padding(20).width(Length::Fill).height(Length::Fill);


        container(main_view)
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn render_chat_area(&self) -> Element<'_, Message> {
        let current_chat = &self.chats[self.current_chat_idx];
        
        // 1. Histórico de Mensagens
        let mut chat_column = column().spacing(12).width(Length::Fill);
        
        for m in &current_chat.messages {
            let is_user = m.starts_with("Tu:");
            let display_text = m.replace("Tu: ", "").replace("Blue Shark: ", "");

            let bubble = container(text(display_text))
                .padding(12)
                .max_width(500.0) // Limita a largura da bolha de chat
                .class(if is_user {
                    cosmic::theme::Container::Secondary // Cor neutra para o utilizador
                } else {
                    cosmic::theme::Container::Primary   // Cor de destaque para a IA
                });

            let mut row_wrapper = row().spacing(0).width(Length::Fill);

            if is_user {
                // Se for o utilizador, empurramos a bolha para a direita com um espaço flexível
                row_wrapper = row_wrapper
                    .push(Space::new().width(Length::Fill))
                    .push(bubble);
            } else {
                // Se for a IA, a bolha fica à esquerda e o espaço fica à direita
                row_wrapper = row_wrapper
                    .push(bubble)
                    .push(Space::new().width(Length::Fill));
            }

            let row_wrapper = row_wrapper.align_y(Alignment::Center);

            chat_column = chat_column.push(row_wrapper);
        }

        let chat_history = scrollable(chat_column)
            .height(Length::Fill)
            .width(Length::Fill);

        // 2. Barra de Input (com os teus botões de emoji)
        let input_box = text_input("Pergunta algo ao Blue Shark...", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(|_| Message::SendMessage)
            .width(Length::Fill)
            .padding(10);

        let send_btn = widget::button::standard("🚀")
            .on_press(Message::SendMessage);

        let bottom_bar = row()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(input_box)
            .push(send_btn);

        // Layout Final da Área de Chat
        container(
            column()
                .spacing(15)
                .push(chat_history)
                .push(bottom_bar)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }


}

impl Application for BlueShark {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.kriolos.BlueShark";

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        
        let app = Self::new(core);
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
                                Err(_) => "Erro: Ollama offline?. execute 'ollama serve'".into(),
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
            Message::ChatOptionsOpen(idx) => {
                if let Some(chat) = self.chats.get(idx) {
                    println!("Opções para o chat: {}", chat.title);
                    // Aqui poderias abrir um Menu ou mudar um estado de edição
                }
                Task::none()
            }
                        Message::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                Task::none()
            }
            _ => Task::none(), // Outras ações (Manager, Attach, Micro) para futura implementação
        }
    }


    fn view(&self) -> Element<'_, Self::Message> {
        let mut content = row();
    
        if self.sidebar_visible {
            content = content.push(self.render_sidebar());
        }
        
        content = content.push(self.render_chat_area());

        column()
            .push(content)
            .into()
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        // Criamos o botão com o emoji correspondente ao estado
        let sidebar_toggle = widget::button::standard(
            if self.sidebar_visible { "◀" } else { "☰" }
        )
        .on_press(Message::ToggleSidebar)
        .padding(5); // Ajuste opcional para alinhar com o menu

        // Podes retornar o botão sozinho ou junto com um Menu Bar
        vec![sidebar_toggle.into()]
    }

}

fn main() -> cosmic::iced::Result {
    let settings = Settings::default();
    cosmic::app::run::<BlueShark>(settings, ())
}
