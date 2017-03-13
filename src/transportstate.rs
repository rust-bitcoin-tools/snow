extern crate rustc_serialize;
extern crate arrayvec;

use error::NoiseError;
use handshakestate::{CipherStates, HandshakeState};

/// A state machine encompassing the transport phase of a Noise session, using the two
/// `CipherState`s (for sending and receiving) that were spawned from the `SymmetricState`'s
/// `Split()` method, called after a handshake has been finished.
///
/// See: http://noiseprotocol.org/noise.html#the-handshakestate-object
pub struct TransportState {
    pub cipherstates: CipherStates,
    initiator: bool,
}

impl TransportState {
    pub fn new(cipherstates: CipherStates, initiator: bool) -> Self {
        TransportState {
            cipherstates: cipherstates,
            initiator: initiator,
        }
    }

    pub fn write_transport_message(&mut self,
                                   payload: &[u8],
                                   message: &mut [u8]) -> Result<usize, NoiseError> {
        let cipher = if self.initiator { &mut self.cipherstates.0 } else { &mut self.cipherstates.1 };
        Ok(cipher.encrypt(payload, message))
    }

    pub fn read_transport_message(&mut self,
                                   payload: &[u8],
                                   message: &mut [u8]) -> Result<usize, NoiseError> {
        let cipher = if self.initiator { &mut self.cipherstates.1 } else { &mut self.cipherstates.0 };
        cipher.decrypt(payload, message).map_err(|_| NoiseError::DecryptError)
    }
}