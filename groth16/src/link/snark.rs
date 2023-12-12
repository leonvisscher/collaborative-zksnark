use core::ops::Neg;

use crate::link::matrix::*;
use ark_ec::PairingEngine;

use ark_ec::{AffineCurve, ProjectiveCurve};

use ark_ff::{One, PrimeField, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};

use ark_std::{marker::PhantomData, rand::Rng, vec, vec::Vec};

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct PP<P: PairingEngine> {
    pub l: usize, // # of rows
    pub t: usize, // # of cols
    pub g1: P::G1Affine,
    pub g2: P::G2Affine,
}

impl<P: PairingEngine> PP<P> {
    pub fn new(l: usize, t: usize, g1: &P::G1Affine, g2: &P::G2Affine) -> PP<P> {
        PP {
            l,
            t,
            g1: g1.clone(),
            g2: g2.clone(),
        }
    }
}

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct EK<P: PairingEngine> {
    pub p: Vec<P::G1Affine>,
}

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct VK<P: PairingEngine> {
    pub c: Vec<P::G2Affine>,
    pub a: P::G2Affine,
}

pub trait SubspaceSnark {
    type KMtx;
    type InVec;
    type OutVec;

    type PP;

    type EK;
    type VK;

    type Proof;

    fn keygen<R: Rng>(rng: &mut R, pp: &Self::PP, m: Self::KMtx) -> (Self::EK, Self::VK);
    fn prove(pp: &Self::PP, ek: &Self::EK, x: &[Self::InVec]) -> Self::Proof;
    fn verify(pp: &Self::PP, vk: &Self::VK, y: &[Self::OutVec], pi: &Self::Proof) -> bool;
}

fn vec_to_g2<P: PairingEngine>(pp: &PP<P>, v: &Vec<P::Fr>) -> Vec<P::G2Affine> {
    v.iter()
        .map(|x| pp.g2.mul(x.into_repr()).into_affine())
        .collect::<Vec<_>>()
}

pub struct PESubspaceSnark<PE: PairingEngine> {
    pairing_engine_type: PhantomData<PE>,
}

// NB: Now the system is for y = Mx
impl<P: PairingEngine> SubspaceSnark for PESubspaceSnark<P> {
    type KMtx = SparseMatrix<P::G1Affine>;
    type InVec = P::Fr;
    type OutVec = P::G1Affine;

    type PP = PP<P>;

    type EK = EK<P>;
    type VK = VK<P>;

    type Proof = P::G1Affine;

    fn keygen<R: Rng>(rng: &mut R, pp: &Self::PP, m: Self::KMtx) -> (Self::EK, Self::VK) {
        let mut k: Vec<P::Fr> = Vec::with_capacity(pp.l);
        for _ in 0..pp.l {
            k.push(P::Fr::rand(rng));
        }

        let a = P::Fr::rand(rng);

        let p = SparseLinAlgebra::<P>::sparse_vector_matrix_mult(&k, &m, pp.t);

        let c = scalar_vector_mult::<P>(&a, &k, pp.l);
        let ek = EK::<P> { p };

        let vk = VK::<P> {
            c: vec_to_g2::<P>(pp, &c),
            a: pp.g2.mul(a.into_repr()).into_affine(),
        };
        (ek, vk)
    }

    fn prove(pp: &Self::PP, ek: &Self::EK, x: &[Self::InVec]) -> Self::Proof {
        assert_eq!(pp.t, x.len());
        inner_product::<P>(x, &ek.p)
    }

    fn verify(pp: &Self::PP, vk: &Self::VK, y: &[Self::OutVec], pi: &Self::Proof) -> bool {
        assert_eq!(pp.l, y.len());

        // check that [x]1T · [C]2 = [π]1 · [a]2

        let mut g1_elements: Vec<<P as PairingEngine>::G1Prepared> = vec![];
        let mut g2_elements = vec![];

        for i in 0..y.len() {
            g1_elements.push(P::G1Prepared::from(y[i]));
            g2_elements.push(P::G2Prepared::from(vk.c[i]));
        }

        g1_elements.push(P::G1Prepared::from(*pi));
        g2_elements.push(P::G2Prepared::from(
            vk.a.into_projective().neg().into_affine(),
        ));

        let mut elements = vec![];

        for i in 0..g1_elements.len() {
            elements.push((g1_elements[i].clone(), g2_elements[i].clone()));
        }

        let lhs = P::product_of_pairings(&elements);
        let rhs = P::Fqk::one();

        lhs == rhs
    }
}
