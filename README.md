![image](https://user-images.githubusercontent.com/38070512/123351255-b495f000-d52a-11eb-8170-fe077b013f01.png)
Photo by [David Zwirner](https://www.davidzwirner.com/viewing-room/2020/lixia)

# Kabocha, smart contract parachain on KSM, from Edgeware.
This is a template repo originally developed by Drew Stone, along with Webb members (Thank you!!)  It is currently being modified by the Edgeware community, along with it's genesis distribution, to suit the use case. 


# Contributing
The parachain repos are coordinated around branches such as `polkadot-v0.9.X` on the Substrate & Cumulus and `release-v0.9.X` on the Polkadot repos respectively. It is good practice that we follow that here, while using `main` to represent the best, currently working version of the node against the most up-to-date upstream versions. Currently, we have our latest work on `polkadot-v0.9.7` and related branches.

## Updating versions
In order to update to something like `polkadot-v0.9.8` and `releaes-v0.9.8` the process is quite simple barring any upstream changes.
1. Convert all references to `v0.9.7` to `v0.9.8`.
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
Ensure you have all relevant dependencies for building the repo. The latest work so far is pegged to `polkadot-v0.9.7` and `release-v0.9.7` branches of Substrate, Cumulus, and Polkadot respectively.
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
git checkout release-v0.9.7
cargo build --release

# generate the relay chain spec
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode > roc-local.json

# run the relay chain with only `--node-key <key>` param and note the peer ID.
# modify the roc-local.json file, bootnodes param with the appropriate IP and peer ID:
# "/ip4/127.0.0.1/tcp/50555/p2p/<peer ID>"

# generate the raw relay chain spec
./target/release/polkadot build-spec --chain=./roc-local.json --disable-default-bootnode --raw > roc-local-raw.json

# generate the collator chainspec
./target/release/hedgeware-collator build-spec --chain hedgeware-config --disable-default-bootnode > hedgeware-local.json

# run the parachain with only `--node-key <key>` param and note the peer ID.
# "/ip4/127.0.0.1/tcp/30333/p2p/<peer ID>"
# modify the hedgeware-local.json file, bootnodes param with the appropriate IP and peer ID:
./target/release/hedgeware-collator build-spec --chain=./hedgeware-local.json --disable-default-bootnode --raw > hedgeware-local.chainspec.json

# generate the collator genesis configs
./target/release/hedgeware-collator export-genesis-state --chain=./hedgeware-local.chainspec.json --parachain-id 2000 > genesis-state-2000
./target/release/hedgeware-collator export-genesis-wasm --chain=./hedgeware-local.chainspec.json > genesis-code-2000

```

# Usage against Rococo spec
```
cargo build --release

# Before generating the spec, reserve the paraID on rococo and modify para ID params to the appropriate para ID.

# generate the spec
./target/release/hedgeware-collator build-spec --chain=hedgeware-config --disable-default-bootnode > hedgeware-rococo.json
./target/release/hedgeware-collator build-spec --chain=./res/hedgeware-rococo.json --disable-default-bootnode --raw > ./res/hedgeware-rococo.chainspec.json

# generate the genesis
./target/release/hedgeware-collator export-genesis-state --chain=./res/hedgeware-rococo.chainspec.json --parachain-id <paraID> > rococo-state
./target/release/hedgeware-collator export-genesis-wasm --chain=./res/hedgeware-rococo.chainspec.json > rococo-code

# Run the collator
./target/release/hedgeware-collator --collator -d /tmp/parachain --node-key <key> --force-authoring --ws-port 9944 --rpc-cors all --parachain-id <rococo-paraID> --port=30333 --chain=./res/hedgeware-rococo.chainspec.json --alice -- --execution wasm --chain rococo -d ~/perm/rococo --port=30334 --ws-port 9945
```

# Register the parachain in the local setup
![image](https://user-images.githubusercontent.com/13153687/122500037-ffaa8300-cfbf-11eb-850d-8d76f51d0722.png)

### Constructing a genesis distribution

```
yarn
ENDPOINT=ws://mainnet2.edgewa.re:9944 node scripts/getHedgewareDistribution.js
```

This will produce a file called allocations.json in the root directory, following
the quaddrop allocation (balances are quadratically mapped and then totaled out to 5m).
You should move it to quaddrop/allocation/dump.json.

You may wish to adjust getHedgewareDistribution.js to reflect any other allocation,
such as a direct hard spoon.
