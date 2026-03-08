pub enum DisconnectReason {
    VICTORY,
    DRAW,
    DISCONNECT,
}

pub enum Extension {
    NAMEDQUEUES,
    NOISE,
    RANK,
    CHAT,
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
