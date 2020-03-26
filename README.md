# decengle-chain

## need
nightly-2020-02-29  
wasm32-unknown-unknown

## start
Download the source code:  
`git clone https://github.com/Lawliet-Chan/bhd-chain.git`  
  
Init Substrate submodule  
`git submodule update --init`  
  
Init rustup toolchain and Build   
`sh scripts/init.sh && cargo build --release`

Start blockchain  

>./target/release/bhd-chain \
 --base-path /tmp/alice \
 --chain=local \
 --alice \
 --telemetry-url ws://telemetry.polkadot.io:1024 \
 --validator
 