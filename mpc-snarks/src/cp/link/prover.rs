#![allow(dead_code)]
use super::data_structures::{Proof, ProvingKey, VerifyingKey};
use super::matrix::inner_product;
use super::r1cs_to_qap::R1CStoQAP;
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{Field, PrimeField, UniformRand, Zero};
use ark_poly::GeneralEvaluationDomain;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, OptimizationGoal, Result as R1CSResult,
};
use ark_std::rand::Rng;
use ark_std::{end_timer, start_timer, vec::Vec};
use log::debug;

// Changelog:
// 1. Specialized to Bls12_377 (our MPC lifting machinery cannot be written fully generically b/c
//    of Rust type system/ ark design limitations).
// 2. Lift to MsmCurve.
// 3. Remove zero-check for prover randomness r.

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Create a Groth16 proof using randomness `r` and `s`.
#[inline]
pub fn create_proof<E, C>(
    circuit: C,
    pk: &ProvingKey<E>,
    r: &[<E as PairingEngine>::Fr],
) -> R1CSResult<Proof<E>>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<<E as PairingEngine>::Fr>,
    <<E as PairingEngine>::Fr as PrimeField>::BigInt: From<<E as PairingEngine>::Fr>,
{
    let a = inner_product::<E>(r, &pk.p);

    Ok(Proof { a })
}

fn calculate_coeff<G: AffineCurve>(
    initial: G::Projective,
    query: &[G],
    vk_param: G,
    assignment: &[G::ScalarField],
) -> G::Projective where {
    let el = query[0];
    let t = start_timer!(|| format!("MSM size {} {}", query.len() - 1, assignment.len()));
    let acc = G::multi_scalar_mul(&query[1..], assignment);
    end_timer!(t);
    let mut res = initial;
    res.add_assign_mixed(&el);
    res += &acc;
    res.add_assign_mixed(&vk_param);

    res
}
