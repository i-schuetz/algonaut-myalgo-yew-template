use algonaut::algod::{v2::Algod, AlgodBuilder};
use my_algo::my_algo::MyAlgo;

use crate::provider::Provider;

pub fn algod() -> Algod {
    AlgodBuilder::new()
        .bind("http://127.0.0.1:53630")
        .auth("44d70009a00561fe340b2584a9f2adc6fec6a16322554d44f56bef9e682844b9")
        .build_v2()
        // expect is acceptable here: as documented on algonaut::algod::v2::AlgodBuilder::build_v2,
        // it returns an error only if the URL or token are not provided or have an invalid format.
        // We are passing previously verified hardcoded values.
        .expect("Couldn't initialize algod")
}

pub fn my_algo() -> MyAlgo {
    MyAlgo {}
}

pub fn provider(algod: Algod, my_algo: MyAlgo) -> Provider {
    Provider::new(algod, my_algo)
}
