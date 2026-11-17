// Integration test for SigmaOS-collabora-online
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_collabora_online as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
