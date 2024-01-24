use ark_ec::PairingEngine;
use ark_ff::bytes::ToBytes;
use ark_serialize::*;
use ark_std::{
    io::{self, Result as IoResult},
    vec::Vec,
};

#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof<P: PairingEngine> {
    pub a: P::G1Affine,
}

impl<E: PairingEngine> ToBytes for Proof<E> {
    #[inline]
    fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        self.a.write(&mut writer)
    }
}

impl<E: PairingEngine> Default for Proof<E> {
    fn default() -> Self {
        Self {
            a: E::G1Affine::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// A verification key in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct VerifyingKey<E: PairingEngine> {
    pub c: Vec<E::G2Affine>,
    pub a: E::G2Affine,
}

impl<E: PairingEngine> ToBytes for VerifyingKey<E> {
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.a.write(&mut writer)?;
        for q in &self.c {
            q.write(&mut writer)?;
        }
        Ok(())
    }
}

impl<E: PairingEngine> Default for VerifyingKey<E> {
    fn default() -> Self {
        Self {
            a: E::G2Affine::default(),
            c: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The prover key for for the Groth16 zkSNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProvingKey<E: PairingEngine> {
    pub p: Vec<E::G1Affine>,
}
