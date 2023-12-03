#![macro_use]
#![feature(associated_type_defaults)]

pub mod reveal;
pub use reveal::Reveal;

pub mod channel;

pub mod com;

pub mod share;
pub use share::SpdzPairingShare;

pub mod wire;
pub use wire::{MpcField, MpcPairingEngine};

pub mod malicious_majority {
    use super::{
        share::spdz::SpdzFieldShare,
        wire::{field, group, pairing},
    };
    pub type MpcField<F> = field::MpcField<F, SpdzFieldShare<F>>;
}
