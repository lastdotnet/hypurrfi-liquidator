use ethers_contract_abigen::Abigen;

fn main() {
    // Generate the bindings
    Abigen::new(
        "Liquidator",                                    // Contract name
        "../liquidator-contract/out/Liquidator.sol/Liquidator.json",                        // ABI location
    )
    .expect("Could not instantiate Abigen")
    .generate()
    .expect("Could not generate bindings")
    .write_to_file("./src/liquidator.rs")    // Write to a file
    .expect("Could not write bindings to file");

    // Tell cargo to rerun this script if the ABI file changes
    println!("cargo:rerun-if-changed=./abi/Liquidator.json");
} 