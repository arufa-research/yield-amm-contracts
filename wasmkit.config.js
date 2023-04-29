

const juno_testnet_accounts = [
  {
    name: 'account_0',
    address: 'juno1evpfprq0mre5n0zysj6cf74xl6psk96gus7dp5',
    mnemonic: 'omit sphere nurse rib tribe suffer web account catch brain hybrid zero act gold coral shell voyage matter nose stick crucial fog judge text'
  },
  {
    name: 'account_1',
    address: 'juno1njamu5g4n0vahggrxn4ma2s4vws5x4w3u64z8h',
    mnemonic: 'student prison fresh dwarf ecology birth govern river tissue wreck hope autumn basic trust divert dismiss buzz play pistol focus long armed flag bicycle'
  }
];

const neutron_testnet_accounts = [
  {
    name: 'admin',
    address: 'neutron1jtdje5vq42sknl22r4wu9sahryu5wcrdqsccjh',
    mnemonic: 'category fine rapid trumpet dune early wish under nothing dance property wreck'
  },
];

const localnet_accounts = [
  {
    name: 'account_0',
    address: '',
    mnemonic: ''
  }
];

const juno_mainnet_accounts = [
];

// Default list covers most of the supported network
// Networks which are not required can be removed from here
const networks = {
  localnet: {
    endpoint: 'http://localhost:26657/',
    chainId: 'testing-1',
    accounts: localnet_accounts,
  },
  juno_testnet: {
    endpoint: 'https://rpc.uni.juno.deuslabs.fi/',
    chainId: 'uni-6',
    accounts: juno_testnet_accounts,
  },
  juno_mainnet: {
    endpoint: 'https://juno-rpc.polkachu.com/',
    chainId: 'juno-1',
    accounts: juno_mainnet_accounts,
  },
  neutron_testnet: {
    endpoint: 'https://rpc.baryon.ntrn.info/',
    chainId: 'baryon-1',
    accounts: neutron_testnet_accounts,
  },
  terra_testnet: {
    endpoint: 'https://terra-testnet-rpc.polkachu.com:443/',
    accounts: terra_testnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "100000", denom: "uluna" }],
        gas: "500000",
      },
      init: {
        amount: [{ amount: "50000", denom: "uluna" }],
        gas: "250000",
      },
      exec: {
        amount: [{ amount: "50000", denom: "uluna" }],
        gas: "250000",
      }
    },
  },
  terra_mainnet: {
    endpoint: 'https://terra-rpc.stakely.io:443/',
    accounts: terra_mainnet_accounts,
  }
};

module.exports = {
  networks: {
    default: networks.neutron_testnet,
    testnet: networks.neutron_testnet,
    localnet: networks.localnet,
    mainnet: networks.juno_mainnet,
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.63.0",
  },
  commands: {
    compile: "RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown",
    schema: "cargo run --example schema",
  }
};