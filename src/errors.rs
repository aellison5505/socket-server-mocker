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
    #[error("{prefix}: Failed to bind TCP listener to {0}: {1}", prefix = self.fatal_str())]
    UnableToBindListener(SocketAddr, io::Error),
    #[error("{prefix}: Failed to get local address of a listener: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToGetLocalAddress(io::Error),
    #[error("{prefix}: Failed to accept incoming connection on {addr}: {source}", prefix = self.fatal_str(), addr = .0, source = .1)]
    UnableToAcceptConnection(SocketAddr, io::Error),
    #[error("{prefix}: Failed to send instructions list to TCP server mocker: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToSendInstructions(SendError<Vec<Instruction>>),
    #[error("{prefix}: Failed to set read timeout on TCP stream: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToSetReadTimeout(io::Error),
    #[error("{prefix}: Failed to read from TCP stream: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToReadTcpStream(io::Error),
    #[error("{prefix}: Failed to write to TCP stream: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToWriteTcpStream(io::Error),
    #[error("{prefix}: Failed to receive message from client: {source}", prefix = self.fatal_str(), source = .0)]
    UnableToReadUdpStream(io::Error),
    #[error("{prefix}: SendMessage instruction received before a ReceiveMessage", prefix = self.fatal_str())]
    GotSendMessageBeforeReceiveMessage,
    #[error("{prefix}: Failed to send message to client: {source}", prefix = self.fatal_str(), source = .0)]
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
