pub mod circuit;

use ark_ec::bls12::Bls12;

use ark_bls12_377::{Fr, Parameters};
use ark_groth16::{generate_random_parameters, prepare_verifying_key, verify_proof, ProvingKey};
use ark_std::test_rng;
use mpc_algebra::reveal::Reveal;
use mpc_net::{MpcMultiNet, MpcNet};
use structopt::StructOpt;

use mpc_snarks::groth::prover::create_random_proof;

use mpc_algebra::{malicious_majority::MpcField, MpcPairingEngine, SpdzPairingShare};

// Local crates
use crate::circuit::VerifyMultiplicationCircuit;

fn start_party(party_id: u8) {
    MpcMultiNet::init_from_file("./data/2", party_id as usize);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "proof", about = "Standard and MPC proofs")]
struct Opt {
    // Party id
    #[structopt(long)]
    party: u8,

    /// Input arguments
    #[structopt()]
    args: Vec<u64>,
}

fn main() {
    /// Function for performing MPC Groth16 proofing with 2 parties using SPDZ MPC protocol.
    let opt = Opt::from_args();
    let party_id = opt.party;

    start_party(party_id);

    type E = Bls12<Parameters>;
    type S = SpdzPairingShare<E>;

    println!("Starting setup...");

    let rng = &mut test_rng();

    let circ_no_data = VerifyMultiplicationCircuit { a: None, b: None };

    let params: ProvingKey<E> = generate_random_parameters::<E, _, _>(circ_no_data, rng).unwrap();

    let pvk = prepare_verifying_key::<E>(&params.vk);

    // ########################################
    // Here the MPC starts
    // ########################################
    let mpc_params = ProvingKey::from_public(params);

    println!("Starting proofing...");

    // User MpcField instead of regular ScalarField
    let inputs = opt
        .args
        .iter()
        .map(|i| MpcField::<Fr>::from_add_shared(Fr::from(*i)))
        .collect::<Vec<_>>();

    println!("party: {}", party_id);

    let a = inputs[0];
    let b = inputs[1];
    let c = inputs[2];

    let mpc_proof = create_random_proof::<MpcPairingEngine<E, S>, _, _>(
        VerifyMultiplicationCircuit {
            a: Some(a),
            b: Some(b),
        },
        &mpc_params,
        rng,
    )
    .unwrap();

    println!("Proof created");

    let proof = mpc_proof.reveal();

    // An error is thrown when .reveal() has different values for different parties
    let pub_c = c.reveal();

    // Assert that verify_proof works with correct value
    assert!(verify_proof(&pvk, &proof, &[pub_c]).unwrap());

    MpcMultiNet::deinit();
}
