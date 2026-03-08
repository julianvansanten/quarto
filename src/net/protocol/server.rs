use crate::net::protocol::{
    commands::{ClientCommand, ServerCommand},
    errors::NetworkError,
};

pub trait ServerProtocol {
    /// Send a given server command over the network.
    fn send_server_command(command: ServerCommand);

    /// Receive a command from the client.
    /// If the data from a client is not a valid command, return a `MalformedNetworkCommand`.
    fn receive_client_command(input: String) -> Result<ClientCommand, NetworkError>;
}
