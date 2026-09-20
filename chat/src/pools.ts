// The pools the tokens trade in, as the Datamine dashboard lists them: one chain heading per
// group, one row per token, and the venue the dashboard itself sends a trader to.

export interface Venue {
  name: string;
  icon: string;
}

export interface PoolToken {
  name: string;
  icon: string;
  venue: Venue;
  trade: string;
  hot?: boolean;
}

export interface PoolChain {
  name: string;
  icon: string;
  tokens: PoolToken[];
}

const uniswap: Venue = { name: 'Uniswap', icon: '/images/uniswap.svg' };
const defiLlama: Venue = { name: 'DefiLlama', icon: '/images/defillama.svg' };

export const poolChains: PoolChain[] = [
  {
    name: 'Ethereum L1',
    icon: '/images/ethereum.svg',
    tokens: [
      { name: 'DAM', icon: '/images/dam48.png', venue: uniswap, trade: 'https://app.uniswap.org/explore/tokens/ethereum/0xF80D589b3Dbe130c270a69F1a69D050f268786Df?chain=multichain' },
      { name: 'FLUX', icon: '/images/flux48.png', venue: uniswap, trade: 'https://app.uniswap.org/explore/tokens/ethereum/0x469eDA64aEd3A3Ad6f868c44564291aA415cB1d9?chain=multichain' },
    ],
  },
  {
    name: 'Arbitrum L2',
    icon: '/images/arbitrum.svg',
    tokens: [
      { name: 'ArbiFLUX', icon: '/images/arbiflux48.png', venue: defiLlama, trade: 'https://swap.defillama.com/?chain=arbitrum&from=0x0000000000000000000000000000000000000000&to=0x64081252c497FCfeC247a664e9D10Ca8eD71b276' },
      { name: 'LOCK', icon: '/images/lock48.png', venue: uniswap, trade: 'https://app.uniswap.org/explore/tokens/arbitrum/0x454F676D44DF315EEf9B5425178d5a8B524CEa03', hot: true },
    ],
  },
];
