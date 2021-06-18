<img width="966" alt="Screen Shot 2021-06-17 at 23 18 11" src="https://user-images.githubusercontent.com/13153687/122501407-a1cb6a80-cfc2-11eb-906a-5894572e64be.png">

# Hedgeware
This is a template "Hedgeware" repo with the proposed Hedgeware distribution outlined in [the announcement post](https://commonwealth.im/edgeware/proposal/discussion/1153). To be honest and up-front this proposal has not seen community consensus and so this shouldn't be expected yet to see the light of day. Nonetheless, it has reached the MOST consensus and this repo serves as the work in progress that is the only currently worked-on parachain node for an Edgeware canary network.

Myself, Drew, and Nathan aren't terribly opinionated about launch, rather exploring actual deployment paths to see parachain work grow in the Edgeware community. We welcome any contributors in the Edgeware community to help test and provide tools towards that goal. We do not guarantee any final output for this project. The node is in its current form ready to be deployed by the right community member.

With that out of the way...

# Contributing
The parachain repos are coordinated around branches such as `polkadot-v0.9.X` on the Substrate & Cumulus and `release-v0.9.X` on the Polkadot repos respectively. It is good practice that we follow that here, while using `main` to represent the best, currently working version of the node against the most up-to-date upstream versions. Currently, we have our latest work on `polkadot-v0.9.4` and related branches.

## Updating versions
In order to update to something like `polkadot-v0.9.5` and `releaes-v0.9.5` the process is quite simple barring any upstream changes.
1. Convert all references to `v0.9.4` to `v0.9.5`.
2. Run cargo update.
3. Build the node and fix any errors.
4. Use the `cumlulus/polkadot-parachains` repo as a starting point to address discrepencies / errors.

## Dealing with Frontier
One of the first errors you will encounter is Frontier. We aim to cover this topic below.

Frontier EVM and its relevant RPCs are still quite far behind in Substrate. They are currently pegged to a branch called `frontier` that last saw an update in March. Therefore, any parachain node needs to use a fork of this and update that Frontier repo to the respective `0.9.X`. Today, we are using the fork in @webb-tools. It has some features also not merged into Frontier, mainly updating the `dynamic-fee-pallet` to the Substrate 3.0 macro structure. This causes noticeable changes to the service file of this node and should be dealt with with care. Any fork can suffice.
1. Create a fork of Frontier or use [@webb-tools/frontier](https://github.com/webb-tools/frontier) as a starting point.
2. Update the references to the relevant `polkadot-v0.9.X` of Substrate (note here we only need to update Substrate branches).
3. Push the fork.
4. Update the Hedgeware Frontier dependencies to the right fork.
5. Rebuild and fix errors as they arise.

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
