use crate::{
    backend::Backend,
    frontend::*,
    ui::style::{Theme, PADDING, SPACING},
};
use iced::{
    button, text_input, Align, Button, Color, Column, Command, Container, Element, Length, Row,
    Space, Subscription, Text, TextInput,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    LoginFieldChanged(String, String),
    // LoginWithSession(Session),
    LoginInitiated,
}

#[derive(Default)]
pub struct LoginScreen {
    login_field_states: HashMap<String, text_input::State>,
    login_button: button::State,

    login_info: HashMap<String, String>,
    /// `None` if not logging out, `Some(restoring_session)` if logging in.
    logging_in: Option<bool>,
    /// The error formatted as a string to be displayed to the user.
    current_error: String,
}

impl LoginScreen {
    pub fn with_logging_in(logging_in: Option<bool>) -> Self {
        Self {
            logging_in,
            ..Self::default()
        }
    }

    pub fn with_error(current_error: String) -> Self {
        Self {
            current_error,
            ..Self::default()
        }
    }

    fn create_login_field(
        &mut self,
        name: String,
        is_sensitive: bool,
        theme: Theme,
    ) -> Element<Message> {
        let mut field = TextInput::new(
            self.login_field_states.get_mut(&name).unwrap(),
            &format!("Enter your {} here...", name),
            self.login_info.get(&name).unwrap(),
            |new_value| Message::LoginFieldChanged(name, new_value),
        )
        .padding(PADDING / 2)
        .style(theme);

        if is_sensitive {
            field = field.password();
        }

        field.into()
    }

    pub fn view(&mut self, theme: Theme, backend: &dyn Backend) -> Element<Message> {
        if let Some(restoring_session) = self.logging_in {
            return Container::new(
                Text::new(if restoring_session {
                    "Restoring session..."
                } else {
                    "Logging in..."
                })
                .size(30),
            )
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme)
            .into();
        }

        let error_text = Text::new(&self.current_error)
            .color(Color::from_rgb8(200, 0, 0))
            .size(18);

        let auth_req = backend.auth_req();
        let mut login_fields = Vec::with_capacity(auth_req.len());
        for (name, is_sensitive) in auth_req {
            login_fields.push(self.create_login_field(name.to_string(), *is_sensitive, theme));
        }

        let login_button = Button::new(&mut self.login_button, Text::new("Login"))
            .on_press(Message::LoginInitiated)
            .style(theme);

        let mut login_widgets = Vec::with_capacity(5);
        login_widgets.push(error_text.into());
        login_widgets.extend(login_fields);
        login_widgets.push(login_button.into());

        let login_panel = Column::with_children(login_widgets)
            .align_items(Align::Center)
            .spacing(SPACING * 3);

        let padded_panel = Row::with_children(vec![
            Space::with_width(Length::FillPortion(3)).into(),
            login_panel.width(Length::FillPortion(4)).into(),
            Space::with_width(Length::FillPortion(3)).into(),
        ])
        .height(Length::Fill)
        .align_items(Align::Center);

        Container::new(padded_panel)
            .style(theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, msg: Message) -> Command<super::Message> {
        match msg {
            Message::LoginFieldChanged(key, new_value) => {
                self.login_info.insert(key, new_value);
            }
            // Message::LoginWithSession(session) => {
            //     async fn try_login(session: Session) -> Result<Client, ClientError> {
            //         let mut client = Client::new_with_session(session)?;
            //         client.initial_sync().await?;

            //         Ok(client)
            //     }

            //     return Command::perform(try_login(session), |result| match result {
            //         Ok(client) => super::Message::LoginComplete(client),
            //         Err(err) => super::Message::MatrixError(Box::new(err)),
            //     });
            // }
            Message::LoginInitiated => {
                // async fn try_login(login_info: LoginInformation) -> Result<Client, ClientError> {
                //     let mut client = Client::new(
                //         &format!("https://{}", login_info.homeserver_domain),
                //         &login_info.username,
                //         &login_info.password,
                //     )
                //     .await?;

                //     client.initial_sync().await?;

                //     Ok(client)
                // }

                // self.logging_in = Some(false);
                // return Command::perform(
                //     try_login(self.login_info.clone()),
                //     |result| match result {
                //         Ok(client) => super::Message::LoginComplete(client),
                //         Err(err) => super::Message::MatrixError(Box::new(err)),
                //     },
                // );
            }
        }
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<super::Message> {
        Subscription::none()
    }

    pub fn on_error(&mut self, error_string: String) {
        self.current_error = error_string;
        self.logging_in = None;
    }
}
