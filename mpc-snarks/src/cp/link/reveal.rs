#![allow(missing_docs)]
use ark_ec::PairingEngine;
use mpc_algebra::{struct_reveal_simp_impl, MpcPairingEngine, PairingShare, Reveal};

use super::data_structures::{Proof, ProvingKey, VerifyingKey};

impl<E: PairingEngine, S: PairingShare<E>> Reveal for Proof<MpcPairingEngine<E, S>> {
    type Base = Proof<E>;
    struct_reveal_simp_impl!(Proof; a);
}

impl<E: PairingEngine, S: PairingShare<E>> Reveal for VerifyingKey<MpcPairingEngine<E, S>> {
    type Base = VerifyingKey<E>;
    struct_reveal_simp_impl!(VerifyingKey;
    c, a);
}

impl<E: PairingEngine, S: PairingShare<E>> Reveal for ProvingKey<MpcPairingEngine<E, S>> {
    type Base = ProvingKey<E>;
    struct_reveal_simp_impl!(ProvingKey;
    p);
}
