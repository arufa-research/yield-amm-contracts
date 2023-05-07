const osmo_testnet_accounts = [
  {
    name: 'account_0',
    address: 'osmo1jtdje5vq42sknl22r4wu9sahryu5wcrdv5z27z',
    mnemonic: 'category fine rapid trumpet dune early wish under nothing dance property wreck'
  },
  {
    name: 'account_1',
    address: 'osmo1ytc8aerh9twa5w5lfn4d76nnj4r2svyzrq89kk',
    mnemonic: 'fatigue renew birth gossip bar suffer peanut expire bulb forest garage upper'
  }
];

const localnet_accounts = [
  {
    name: 'account_0',
    address: '',
    mnemonic: ''
  }
];

const osmo_mainnet_accounts = [
  {
    name: 'account_0',
    address: '',
    mnemonic: ''
  }
];

// Default list covers most of the supported network
// Networks which are not required can be removed from here
const networks = {
  localnet: {
    endpoint: 'http://localhost:26657/',
    chainId: 'testing-1',
    accounts: localnet_accounts,
  },
  osmo_testnet: {
    endpoint: 'https://rpc.testnet.osmosis.zone/',
    chainId: 'osmosis-1',
    accounts: osmo_testnet_accounts,
  },
  osmo_mainnet: {
    endpoint: 'https://rpc.osmosis.zone/',
    chainId: 'osmo-test-5',
    accounts: osmo_mainnet_accounts,
  },
  // terra_testnet: {
  //   endpoint: 'https://terra-testnet-rpc.polkachu.com:443/',
  //   accounts: terra_testnet_accounts,
  //   fees: {
  //     upload: {
  //       amount: [{ amount: "100000", denom: "uluna" }],
  //       gas: "500000",
  //     },
  //     init: {
  //       amount: [{ amount: "50000", denom: "uluna" }],
  //       gas: "250000",
  //     },
  //     exec: {
  //       amount: [{ amount: "50000", denom: "uluna" }],
  //       gas: "250000",
  //     }
  //   },
  // }
};

module.exports = {
  networks: {
    default: networks.osmo_testnet,
    testnet: networks.osmo_testnet,
    localnet: networks.localnet,
    mainnet: networks.osmo_mainnet,
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.63.0",
  },
  commands: {
    compile: "cargo wasm",
    schema: "cargo schema",
  }
};