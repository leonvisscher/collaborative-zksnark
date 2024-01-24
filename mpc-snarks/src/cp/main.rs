pub mod circuit;
pub mod commitment;
pub mod link;
pub mod multiply;
pub mod test_groth;
// pub mod test_lego;
pub mod test_link;

use crate::test_groth::test_groth;
use crate::test_lego::test_lego;
use crate::test_link::test_link;
use ark_bls12_377::{Bls12_377, Parameters};
use ark_ec::bls12::Bls12;
use mpc_algebra::{MpcPairingEngine, SpdzPairingShare};
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
    type E = Bls12_377;
    type S = SpdzPairingShare<E>;

    // test_groth();
    test_link::<Bls12_377, MpcPairingEngine<E, S>>();
    // test_lego();
}

// Add test for generating Pedersen commitments
// Collaborative subspace circuit
// Test and document how we "collaborate" on the Pedersen commitments (i.e. we assume a shared vector? But how do we obtain it in the first place?) We can use from_add_shared to instantieat the shares, but how to get these values in the first place?
// ... Add more and more until we have LegoGroth16
