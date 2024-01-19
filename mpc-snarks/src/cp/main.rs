pub mod circuit;
pub mod commitment;
pub mod cp_link;
pub mod groth16;
pub mod link;
pub mod multiply;

use crate::commitment::test_collaborative_commitment;
use crate::cp_link::test_collaborative_link;
use crate::groth16::test_mpc_groth16;
use crate::multiply::test_collaborative_mul;
use ark_bls12_377::Parameters;
use ark_ec::bls12::Bls12;
use mpc_algebra::SpdzPairingShare;
use structopt::StructOpt;

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
    type E = Bls12<Parameters>;
    type S = SpdzPairingShare<E>;

    // test_mpc_groth16();
    // test_collaborative_mul();
    // test_collaborative_link::<E>();
    test_collaborative_commitment::<E>();
}

// Add test for generating Pedersen commitments
// Collaborative subspace circuit
// Test and document how we "collaborate" on the Pedersen commitments (i.e. we assume a shared vector? But how do we obtain it in the first place?) We can use from_add_shared to instantieat the shares, but how to get these values in the first place?
// ... Add more and more until we have LegoGroth16
