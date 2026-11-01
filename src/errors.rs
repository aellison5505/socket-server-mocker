//! # `server_mocker_error`
//!
//! `server_mocker_error` is a type representing an error raised by a server mocker. It's mainly errors raised by the underlying socket server.
//!
//! The error is raised directly during call to [`ServerMocker`](crate::ServerMocker) methods, or when the server mocker is running asynchronously and an error occurs.
//!
//! If so, errors can be retrieved with [`ServerMocker::pop_server_error`](crate::ServerMocker::pop_server_error) method.

use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::SendError;

use crate::Instruction;

/// Represents an error raised by a server mocker.
///
/// The error is raised directly during call to [`ServerMocker`](crate::ServerMocker) methods, or when the server mocker is running asynchronously and an error occurs.
/// If so, errors can be retrieved with [`ServerMocker::pop_server_error`](crate::ServerMocker::pop_server_error) method.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum ServerMockerError {
    #[error("{fatal}: Failed to bind TCP listener to {address}: {source}", fatal = self.fatal_str(), address = .0, source = .1)]
    UnableToBindListener(SocketAddr, io::Error),
    #[error("{fatal}: Failed to get local address of a listener: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToGetLocalAddress(io::Error),
    #[error("{fatal}: Failed to accept incoming connection on {address}: {source}", fatal = self.fatal_str(), address = .0, source = .1)]
    UnableToAcceptConnection(SocketAddr, io::Error),
    #[error("{fatal}: Failed to send instructions list to TCP server mocker: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToSendInstructions(SendError<Vec<Instruction>>),
    #[error("{fatal}: Failed to set read timeout on TCP stream: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToSetReadTimeout(io::Error),
    #[error("{fatal}: Failed to read from TCP stream: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToReadTcpStream(io::Error),
    #[error("{fatal}: Failed to write to TCP stream: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToWriteTcpStream(io::Error),
    #[error("{fatal}: Failed to receive message from client: {error}", fatal = self.fatal_str(), error = .0)]
    UnableToReadUdpStream(io::Error),
    #[error("{fatal}: SendMessage instruction received before a ReceiveMessage", fatal = self.fatal_str())]
    GotSendMessageBeforeReceiveMessage,
    #[error("{fatal}: Failed to send message to client: {error}", fatal = self.fatal_str(), error = .0)]
    FailedToSendUdpMessage(io::Error),
}

impl ServerMockerError {
    /// Indicate if this is a fatal error
    pub fn is_fatal(&self) -> bool {
        match self {
            ServerMockerError::UnableToBindListener(_, _)
            | ServerMockerError::UnableToGetLocalAddress(_)
            | ServerMockerError::UnableToAcceptConnection(_, _)
            | ServerMockerError::UnableToSetReadTimeout(_) => true,

            ServerMockerError::UnableToSendInstructions(_)
            | ServerMockerError::UnableToReadTcpStream(_)
            | ServerMockerError::UnableToWriteTcpStream(_)
            | ServerMockerError::UnableToReadUdpStream(_)
            | ServerMockerError::GotSendMessageBeforeReceiveMessage
            | ServerMockerError::FailedToSendUdpMessage(_) => false,
        }
    }

    fn fatal_str(&self) -> &'static str {
        if self.is_fatal() {
            "Fatal"
        } else {
            "Non fatal"
        }
    }
}
