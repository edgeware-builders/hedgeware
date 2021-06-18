# hedgeware

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
