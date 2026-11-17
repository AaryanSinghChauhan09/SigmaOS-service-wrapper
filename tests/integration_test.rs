// Integration test for SigmaOS-mint-service-wrapper
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use mint_service_wrapper as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
