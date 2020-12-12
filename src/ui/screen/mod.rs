use crate::{backend::Backend, frontend::*, ui::style::Theme};
use iced::{executor, Application, Command, Element, Subscription};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum UiMessage {
    ActionResult(Result<Reaction, PendingAction>),
    /// Sent when a logout request is completed successfully.
    // LogoutComplete,
    /// Sent whenever an error occurs.
    Error(String),
    /// Sent when the "login" is complete, ie. establishing a session and performing an initial sync.
    // LoginComplete,
    /// Do nothing.
    Nothing,
}

#[derive(Debug)]
pub enum StartupFlag {
    /// Use this session to login and skip the login screen.
    // UseSession(Session),
    None,
}

impl Default for StartupFlag {
    fn default() -> Self {
        Self::None
    }
}

impl Display for StartupFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // StartupFlag::UseSession(session) => {
            //     write!(f, "Use this session when logging in: {}", session)
            // }
            StartupFlag::None => write!(f, "No flag"),
        }
    }
}

pub struct Main {
    theme: Theme,
    backend: Box<dyn Backend>,
}

impl Application for Main {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = StartupFlag;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        match flags {
            // // "Login" with given session, skipping the info fields.
            // StartupFlag::UseSession(session) => (
            //     Self {
            //         screen: Screen::Login {
            //             screen: LoginScreen::with_logging_in(Some(true)),
            //         },
            //         ..Self::default()
            //     },
            //     Command::perform(async { session }, |session| {
            //         Message::LoginScreen(login::Message::LoginWithSession(session))
            //     }),
            // ),
            StartupFlag::None => (Self::default(), Command::none()),
        }
    }

    fn title(&self) -> String {
        String::from("Icy Matrix")
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Nothing => {}
            Message::ActionResult(result) => match result {
                Ok(reaction) => match reaction {
                    Reaction::LoggedOut => {}
                    Reaction::AuthComplete { user } => {
                        //
                    }
                    Reaction::MessageSent { node_id, msg_id } => {
                        // mark message as sent
                    }
                },
                Err(pending) => {
                    let PendingAction {
                        action,
                        remaining_repeat_count,
                        repeat_after,
                        error,
                    } = pending;

                    remaining_repeat_count -= 1;
                    if remaining_repeat_count < 0 {
                        const ERR_MSG: &str =
                            "An operation failed and was discarded since it was tried many times";
                        let error_msg_formatted = error.map_or_else(
                            || format!("{}.", ERR_MSG),
                            |error_msg| format!("{}: {}", ERR_MSG, error_msg),
                        );

                        return self.update(UiMessage::Error(error_msg_formatted));
                    }

                    let err_cmd = error.map(|error_msg| self.update(UiMessage::Error(error_msg)));
                }
            },
            // Message::LoginComplete => {
            //     self.screen = Screen::Main {
            //         screen: MainScreen::default(),
            //     };
            // }
            // Message::LogoutComplete => {
            //     self.screen = Screen::Login {
            //         screen: LoginScreen::default(),
            //     };
            // }
            Message::Error(reason) => {
                // use ruma::{api::client::error::ErrorKind as ClientAPIErrorKind, api::error::*};
                // use ruma_client::Error as InnerClientError;

                log::error!("{}", reason);

                // if let ClientError::Internal(err) = *err {
                //     if let InnerClientError::FromHttpResponse(err) = err {
                //         if let FromHttpResponseError::Http(err) = err {
                //             if let ServerError::Known(err) = err {
                //                 // Return to login screen since the users session has expired.
                //                 if let ClientAPIErrorKind::UnknownToken { soft_logout: _ } =
                //                     err.kind
                //                 {
                //                     self.screen = Screen::Login {
                //                         screen: LoginScreen::with_error(error_string),
                //                     };

                //                     return Command::none();
                //                 }
                //             }
                //         }
                //     }
                // }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::from_recipe(self.backend.reaction_stream()).map(Message::Reaction)
    }

    fn view(&mut self) -> Element<Self::Message> {}
}
