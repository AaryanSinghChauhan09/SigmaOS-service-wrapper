//! SigmaOS-mint-service-wrapper — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_mint_service_wrapper_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-mint-service-wrapper: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-mint-service-wrapper: ok");
}
