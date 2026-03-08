use std::fmt;

const DELIM: &str = "~";

pub enum DisconnectReason {
    VICTORY,
    DRAW,
    DISCONNECT,
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisconnectReason::VICTORY => write!(f, "VICTORY"),
            DisconnectReason::DRAW => write!(f, "DRAW"),
            DisconnectReason::DISCONNECT => write!(f, "DISCONNECT"),
        }
    }
}

pub enum Extension {
    NAMEDQUEUES,
    NOISE,
    RANK,
    CHAT,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Extension::NAMEDQUEUES => write!(f, "NAMEDQUEUES"),
            Extension::NOISE => write!(f, "NOISE"),
            Extension::RANK => write!(f, "RANK"),
            Extension::CHAT => write!(f, "CHAT"),
        }
    }
}

pub enum ServerCommand {
    HELLO {
        description: String,
        extensions: Vec<Extension>,
    },
    LOGIN,
    ALREADYLOGGEDIN,
    LIST {
        usernames: Vec<String>,
    },
    NEWGAME {
        player1: String,
        player2: String,
    },
    MOVE {
        n: u8,
        m: Option<u8>,
    },
    GAMEOVER {
        reason: DisconnectReason,
        winner: Option<String>,
    },
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerCommand::HELLO {
                description,
                extensions,
            } => {
                let mut extension_string = String::new();
                for extension in extensions {
                    extension_string.push_str(DELIM);
                    extension_string = extension_string + &extension.to_string();
                }
                write!(f, "HELLO{}{}{}", DELIM, description, extension_string)
            }
            ServerCommand::LOGIN => write!(f, "LOGIN"),
            ServerCommand::ALREADYLOGGEDIN => write!(f, "ALREADYLOGGEDIN"),
            ServerCommand::LIST { usernames } => {
                let mut usernames_string = String::new();
                for user in usernames {
                    usernames_string.push_str(DELIM);
                    usernames_string = usernames_string + user;
                }
                write!(f, "LIST{}", usernames_string)
            }
            ServerCommand::NEWGAME { player1, player2 } => {
                write!(f, "NEWGAME{}{}{}{}", DELIM, player1, DELIM, player2)
            }
            ServerCommand::MOVE { n, m } => match m {
                Some(m_val) => write!(f, "MOVE{}{}{}{}", DELIM, n, DELIM, m_val),
                None => write!(f, "MOVE{}{}", DELIM, n),
            },
            ServerCommand::GAMEOVER { reason, winner } => match winner {
                Some(the_winner) => write!(f, "GAMEOVER{DELIM}{reason}{DELIM}{the_winner}"),
                None => write!(f, "GAMEOVER{DELIM}{reason}"),
            },
        }
    }
}

pub enum ClientCommand {
    HELLO {
        description: String,
        extensions: Vec<Extension>,
    },
    LOGIN {
        name: String,
    },
    LIST,
    QUEUE,
    MOVE {
        n: u8,
        m: Option<u8>,
    },
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientCommand::HELLO {
                description,
                extensions,
            } => {
                let mut extension_string = String::new();
                for extension in extensions {
                    extension_string.push_str(DELIM);
                    extension_string = extension_string + &extension.to_string();
                }
                write!(f, "HELLO{}{}{}", DELIM, description, extension_string)
            }
            ClientCommand::LOGIN { name } => write!(f, "LOGIN{DELIM}{name}"),
            ClientCommand::LIST => write!(f, "LIST"),
            ClientCommand::QUEUE => write!(f, "QUEUE"),
            ClientCommand::MOVE { n, m } => match m {
                Some(m_val) => write!(f, "MOVE{}{}{}{}", DELIM, n, DELIM, m_val),
                None => write!(f, "MOVE{}{}", DELIM, n),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::net::protocol::{ClientCommand, DisconnectReason, Extension, ServerCommand};

    #[test]
    fn test_disconnect_reason_string() {
        assert_eq!(
            DisconnectReason::DISCONNECT.to_string(),
            String::from("DISCONNECT")
        );
        assert_eq!(DisconnectReason::DRAW.to_string(), String::from("DRAW"));
        assert_eq!(
            DisconnectReason::VICTORY.to_string(),
            String::from("VICTORY")
        );
    }

    #[test]
    fn test_extension_string() {
        assert_eq!(Extension::CHAT.to_string(), String::from("CHAT"));
        assert_eq!(
            Extension::NAMEDQUEUES.to_string(),
            String::from("NAMEDQUEUES")
        );
        assert_eq!(Extension::NOISE.to_string(), String::from("NOISE"));
        assert_eq!(Extension::RANK.to_string(), String::from("RANK"));
    }

    #[test]
    fn test_server_command_string() {
        assert_eq!(
            ServerCommand::ALREADYLOGGEDIN.to_string(),
            String::from("ALREADYLOGGEDIN")
        );
        assert_eq!(ServerCommand::LOGIN.to_string(), String::from("LOGIN"));
        assert_eq!(
            ServerCommand::GAMEOVER {
                reason: DisconnectReason::DRAW,
                winner: None
            }
            .to_string(),
            String::from("GAMEOVER~DRAW")
        );
        assert_eq!(
            ServerCommand::GAMEOVER {
                reason: DisconnectReason::VICTORY,
                winner: Some(String::from("player1"))
            }
            .to_string(),
            String::from("GAMEOVER~VICTORY~player1")
        );
        assert_eq!(
            ServerCommand::HELLO {
                description: String::from("test"),
                extensions: vec![]
            }
            .to_string(),
            String::from("HELLO~test")
        );
        assert_eq!(
            ServerCommand::HELLO {
                description: String::from("test"),
                extensions: vec![Extension::CHAT, Extension::RANK]
            }
            .to_string(),
            String::from("HELLO~test~CHAT~RANK")
        );
        assert_eq!(
            ServerCommand::LIST { usernames: vec!() }.to_string(),
            String::from("LIST")
        );
        assert_eq!(
            ServerCommand::LIST {
                usernames: vec![String::from("first"), String::from("second")]
            }
            .to_string(),
            String::from("LIST~first~second")
        );
        assert_eq!(
            ServerCommand::MOVE { n: 5, m: None }.to_string(),
            String::from("MOVE~5")
        );
        assert_eq!(
            ServerCommand::MOVE { n: 5, m: Some(5) }.to_string(),
            String::from("MOVE~5~5")
        );
        assert_eq!(
            ServerCommand::NEWGAME {
                player1: String::from("player1"),
                player2: String::from("player2")
            }
            .to_string(),
            String::from("NEWGAME~player1~player2")
        )
    }

    #[test]
    fn test_client_command_print() {
        assert_eq!(ClientCommand::LIST.to_string(), String::from("LIST"));
        assert_eq!(ClientCommand::QUEUE.to_string(), String::from("QUEUE"));
        assert_eq!(
            ClientCommand::HELLO {
                description: String::from("some_description"),
                extensions: vec!()
            }
            .to_string(),
            String::from("HELLO~some_description")
        );
        assert_eq!(
            ClientCommand::HELLO {
                description: String::from("some_description"),
                extensions: vec![Extension::RANK, Extension::CHAT]
            }
            .to_string(),
            String::from("HELLO~some_description~RANK~CHAT")
        );
        assert_eq!(
            ClientCommand::LOGIN {
                name: String::from("my_name")
            }
            .to_string(),
            String::from("LOGIN~my_name")
        );
        assert_eq!(
            ClientCommand::MOVE { n: 5, m: None }.to_string(),
            String::from("MOVE~5")
        );
        assert_eq!(
            ClientCommand::MOVE { n: 5, m: Some(5) }.to_string(),
            String::from("MOVE~5~5")
        );
    }
}
