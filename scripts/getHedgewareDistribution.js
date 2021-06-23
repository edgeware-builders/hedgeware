const { spec } = require('@edgeware/node-types');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { AccountInfo, AccountData } = require('@polkadot/types/interfaces');
const BN = require('bn.js');
const BigNumber = require('bignumber.js');
const fs = require('fs');

// set bignumber configuration for correct calculations
BigNumber.config({ ROUNDING_MODE: BigNumber.ROUND_DOWN });
BigNumber.config({ DECIMAL_PLACES: 0 });

const ENDPOINT = process.env.ENDPOINT
      || 'ws://mainnet1.edgewa.re:9944';
const DISTRIBUTION_HASH = process.env.DISTRIBUTION_HASH
      || '0x04d0a2f19c6a60e0fcb57e5eb52c431f27010522ded6b0f4b5e571800723cd74';

async function main() {
  console.log('Connecting to', ENDPOINT);
  const provider = new WsProvider([ENDPOINT]);
  // const opts = optionsWithEdgeware({ provider });
  const api = await ApiPromise.create({
    provider,
    ...spec,
  });
  await api.isReady;

  const entries = await api.query.system.account.entriesAt(DISTRIBUTION_HASH);
  let quadSum = new BN('0');
  // get sums and square root sums
  entries.forEach((entry) => {
    const accountInfo = entry[1];
    const accountData = accountInfo.data;
    // sum free + reserved
    const accountBalance = new BN(accountData.free.toString()).add(new BN(accountData.reserved.toString()));
    // take sqrt
    const sqrtOfBalance = new BigNumber(accountBalance.toString()).sqrt();
    // add values to sum
    quadSum = quadSum.add(new BN(sqrtOfBalance.toString()));
  });

  let totalDistributed = new BN('0');
  const distribution = entries.map((entry) => {
    const accountInfo = entry[1];
    const accountData = accountInfo.data;
    // sum free + reserved
    const accountBalance = new BN(accountData.free.toString()).add(new BN(accountData.reserved.toString()));
    // take sqrt
    const sqrtOfBalance = new BigNumber(accountBalance.toString()).sqrt();
    // weight properly for distribution
    const quadraticAllocation = new BN(sqrtOfBalance.toString())
      // multiply by 2,500,000 tokens w/ 18 decimals -> 24 zeros
      .mul(new BN('2500000000000000000000000'))
      .div(quadSum);

    totalDistributed = totalDistributed.add(quadraticAllocation);
    return [entry[0].toString().substr(entry[0].toString().length - 64), quadraticAllocation.toString()];
  });

  fs.writeFileSync('./distribution.json', JSON.stringify({ balances: distribution }, null, 4));
  console.log('Total distributed', totalDistributed.toString());
  await api.disconnect();
}

main().catch((err) => console.log(err));
