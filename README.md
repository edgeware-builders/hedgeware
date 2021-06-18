# hedgeware
This is a template "Hedgeware" repo with the proposed Hedgeware distribution outlined in [the announcement post](https://commonwealth.im/edgeware/proposal/discussion/1153). To be honest and up-front this proposal has not seen community consensus and so this shouldn't be expected yet to see the light of day. Nonetheless, it has reached the MOST consensus and this repo serves as the work in progress that is the only currently worked-on parachain node for an Edgeware canary network.

We aren't terribly opinionated about launch, rather exploring actual deployment paths to see parachain work grow in the Edgeware community. We welcome any contributors in the Edgeware community to help test and provide tools towards that goal. We do not guarantee any final output for this project. The node is in its current form ready to be deployed by the right community member.

With that out of the way...

# Build & generate the local parachain specs
Ensure you have all relevant dependencies for building the repo. The latest work so far is pegged to `polkadot-v0.9.4` and `release-v0.9.4` branches of Substrate, Cumulus, and Polkadot respectively.
```
cargo build --release

./target/release/hedgeware-collator build-spec --chain=hedgeware-config > hedgeware.json
./target/release/hedgeware-collator build-spec --chain=hedgeware.json --raw > hedgeware.chainspec.json
./target/release/hedgeware-collator export-genesis-wasm --chain=hedgeware.chainspec.json > genesis-wasm
./target/release/hedgeware-collator export-genesis-state --chain=hedgeware.chainspec.json --parachain-id 2000 > genesis-state
```
Optionally, for faster setup you can run:
```
cargo build --release
./scripts/build_specs.sh
```
To run against your local chainspec:
```
# Run vanilla execution (requires having keys set through separate means)
./target/release/hedgeware-collator --collator --chain=hedgeware.chainspec.json

# Execute as Alice (default aura collator)
./target/release/hedgeware-collator --alice --collator --chain=hedgeware.chainspec.json

# Execute as Bob (default aura collator)
./target/release/hedgeware-collator --bob --collator --chain=hedgeware.chainspec.json

# If running with 1 collator
./target/release/hedgeware-collator --alice --collator --chain=hedgeware.chainspec.json --force-authoring
```

# Using against a local relay chain
```
# Compile Polkadot with the real overseer feature
git clone https://github.com/paritytech/polkadot
git fetch
git checkout release-v0.9.4
cargo build --release

# Generate a raw chain spec
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json

# Alice
./target/release/polkadot --chain rococo-local-cfde.json --alice --tmp

# Bob (In a separate terminal)
./target/release/polkadot --chain rococo-local-cfde.json --bob --tmp --port 30334
```

# Usage against Rococo spec [work in progress]
```
cargo build --release

./target/release/hedgeware-collator --collator --chain=hedgeware-rococo --pruning=archive
```

# Register the parachain in the local setup
![image](https://user-images.githubusercontent.com/13153687/122500037-ffaa8300-cfbf-11eb-850d-8d76f51d0722.png)
