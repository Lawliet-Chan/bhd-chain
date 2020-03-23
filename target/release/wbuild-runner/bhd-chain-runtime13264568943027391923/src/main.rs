
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/Users/ChenXinRan/rust/src/bhd-chain/target/release/build/bhd-chain-runtime-b1bbc97b90536950/out/wasm_binary.rs",
						"/Users/ChenXinRan/rust/src/bhd-chain/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base ",
					)
				}
			