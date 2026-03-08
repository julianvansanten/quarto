use crate::net::protocol::{
    errors::NetworkError,
    commands::{ClientCommand, ServerCommand},
};

pub trait ClientProtocol {
    /// Send a client command to the server.
    fn send_client_command(command: ClientCommand);

    /// Parse a message from the server.
    /// If the server does not provide a valid `ServerCommand`, respond with a `MalformedNetworkCommand`.
    fn receive_server_command(input: String) -> Result<ServerCommand, NetworkError>;
}
