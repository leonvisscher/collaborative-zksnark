use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct PP<
    G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
> {
    pub l: usize, // # of rows
    pub t: usize, // # of cols
    pub g1: G1,
    pub g2: G2,
}

impl<
        G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
        G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    > PP<G1, G2>
{
    pub fn new(l: usize, t: usize, g1: &G1, g2: &G2) -> PP<G1, G2> {
        PP {
            l,
            t,
            g1: g1.clone(),
            g2: g2.clone(),
        }
    }
}

pub fn vec_to_g2<P: PairingEngine>(
    pp: &PP<P::G1Affine, P::G2Affine>,
    v: &Vec<P::Fr>,
) -> Vec<P::G2Affine>
where
    <<P as PairingEngine>::Fr as PrimeField>::BigInt: From<<P as PairingEngine>::Fr>,
{
    v.iter()
        .map(|x| pp.g2.mul(*x).into_affine())
        .collect::<Vec<_>>()
}
