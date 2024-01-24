use crate::link::circuit::VerifyMultiplicationCircuit;
use crate::link::matrix::SparseMatrix;
use crate::link::snark::PP;
use crate::Opt;

use ark_bls12_377::{Fr, Parameters};
use ark_ec::bls12::Bls12;
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{One, PrimeField, UniformRand, Zero};
use ark_std::test_rng;
use mpc_algebra::reveal::Reveal;
use mpc_algebra::{malicious_majority::MpcField, MpcPairingEngine, SpdzPairingShare};
use mpc_net::{MpcMultiNet, MpcNet};
use structopt::StructOpt;

use crate::link::prover::create_proof;
use crate::link::{data_structures::ProvingKey, generator::keygen, verifier::verify_proof};

pub fn test_link<P: PairingEngine, WrappedP: PairingEngine>()
where
    <<P as PairingEngine>::Fr as PrimeField>::BigInt: From<<P as PairingEngine>::Fr>,
{
    let opt = Opt::from_args();
    let party_id = opt.party;

    MpcMultiNet::init_from_file("./data/2", party_id as usize);

    let rng = &mut test_rng();

    let g1 = P::G1Projective::rand(rng).into_affine();
    let g2 = P::G2Projective::rand(rng).into_affine();
    let mut pp = PP::<P::G1Affine, P::G2Affine> { l: 1, t: 2, g1, g2 };

    let mut m = SparseMatrix::new(1, 2);
    m.insert_row_slice(0, 0, &vec![g1, g1]);

    let x: Vec<P::Fr> = vec![P::Fr::one(), P::Fr::zero()];

    let y: Vec<P::G1Affine> = vec![g1];

    let (ek, vk) = keygen::<P>(rng, &pp, m);

    MpcMultiNet::deinit();
}
