import { spec } from '@edgeware/node-types';
import { WsProvider, ApiPromise } from '@polkadot/api';
import { TypeRegistry, StorageKey } from '@polkadot/types';
import { AccountInfo } from '@polkadot/types/interfaces';
import BN from 'bn.js';
import BigNumber from 'bignumber.js';
BigNumber.config({ DECIMAL_PLACES: 0 })
import { migrateRefcount } from './migrate';

const urls: { [name: string]: string } = {
  'mainnet': 'ws://mainnet1.edgewa.re:9944',
  // 'mainnet': 'ws://localhost:9944',
  'beresheet': 'wss://beresheet1.edgewa.re',
  'dev': 'ws://localhost:9944',
};

const getBalances = async (url: string, blockNumber?: number) => {
  // connect to chain via provider
  console.log(`Connecting to url: ${url}...`);

  // construct API using provider
  console.log('Constructing API...');
  const registry = new TypeRegistry();
  const api = new ApiPromise({
    provider: new WsProvider(url),
    registry,
    ...spec,
  });

  await api.isReady;
  const lastHdr = await api.rpc.chain.getHeader();
  const startHdr = await api.rpc.chain.getBlockHash();
  const entries = await api.query.system.account.entries();
  let sum = new BN('0');
  let quadraticSum = new BN('0');
  entries.forEach(([key, value]) => {
    const sqrtOfFree = (new BigNumber(value.data.free.toString())).sqrt();
    // add values to sum
    sum = sum.add(value.data.free);
    quadraticSum = quadraticSum.add(new BN(sqrtOfFree.toString()));
  });

  let totalQuadAlloc = new BN('0');
  const json = entries.map(([key, value]) => {
    const sqrtOfFree = (new BigNumber(value.data.free.toString())).sqrt();
    const quadraticAllocation = (new BN(sqrtOfFree.toString()))
      // multiply by 5,000,000 tokens w/ 18 decimals -> 24 zeros
      .mul(new BN('5000000000000000000000000'))
      .div(quadraticSum);
    totalQuadAlloc = totalQuadAlloc.add(quadraticAllocation);
    return {
      key: key.toJSON(),
      value: {
        ...value,
        data: {
          free: quadraticAllocation.toString(),
          reserved: '0',
          miscFrozen: '0',
          feeFrozen: '0'
        }
      }
    }
  });
  console.log(sum.toString());
  console.log(quadraticSum.toString());
  console.log(totalQuadAlloc.toString());
  const updatedData = migrateRefcount(json);

  // const encodedData = updatedData.map((elt: any) => {
  //   return {
  //     key: api.createType('StorageKey', elt.key),
  //     value: api.createType('AccountInfo', elt.value),
  //   };
  // });

  require('fs').writeFileSync('dump/accounts.json', JSON.stringify(updatedData, null, 4));
};

// parse args
const args = process.argv.slice(2);
const network = args[0] || 'mainnet';
const url = urls[network];
const block = args[1];

// kick off function
getBalances(url, block ? +block : undefined)
.then(() => {
  console.log('Done!');
  process.exit(0);
}).catch((err) => {
  console.error(err.message);
  process.exit(1);
});