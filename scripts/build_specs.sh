./target/release/hedgeware-collator build-spec --chain=hedgeware-config > hedgeware.json
./target/release/hedgeware-collator build-spec --chain=hedgeware.json --raw > hedgeware.chainspec.json
./target/release/hedgeware-collator export-genesis-wasm --chain=hedgeware.chainspec.json > genesis-wasm
./target/release/hedgeware-collator export-genesis-state --chain=hedgeware.chainspec.json --parachain-id 2000 > genesis-state
