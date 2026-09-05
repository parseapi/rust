// Build the frozen contract from a separate application dependency graph.
// Its fresh lockfile uses normal resolution on the supported minimum Rust.
#[cfg(test)]
#[path = "../../contract.rs"]
mod contract;
