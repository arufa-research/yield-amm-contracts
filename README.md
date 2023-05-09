# FixedFi: Yield AMM contracts

<img src="./assets/FixedFiLogoColor.png" width=25% height=25%>

FixedFi is a DeFi protocol built using CosmWasm contracts that let anyone access to fixed yield on their assets.

Split a yield bearing asset into principle token (pToken) and yield token (yToken) and yToken can be sold at a fixed price to fix the future yield and get it now. 

Users can even buy the pToken at discount w.r.t. undelying asset to get fixed yield without holding a yield bearing asset or having exposure to the yield or source of yield (the protocol).

# Contracts

| Name                                                       | Description                                            |
| ---------------------------------------------------------- | ------------------------------------------------------ |
| [`mars-adapter`](contracts/mars-adapter)                   | Deposits asset into red bank and issues ybToken        |
| [`splitter`](contracts/splitter)                           | Splits ybToken into pToken and yToken and vice-versa   |
| [`yield-bearing-token`](contracts/yield-bearing-token)     | CW20 yield bearing token                               |
| [`principle-token`](contracts/principle-token)             | CW20 principle token to get underlying at discount     |
| [`yield-token`](contracts/yield-token)                     | Cw20 yield token to only get yield exposure            |
| [`swap-router`](contracts/swap-router)                     | Multi-hop trade router                                 |
| [`swap-pair`](contracts/swap-pair)                         | Pair with x*y=k curve (will be updated to stable pair) |
| [`swap-factory`](contracts/swap-factory)                   | Pool creation factory                                  |

# User stories

# Compiling contracts

Use this command to compile your contracts: 

```bash
$ wasmkit compile
```

# Running script

```bash
$ wasmkit run scripts/sample-script.ts
```

# Deploying contracts

In `scripts/` directory:

First of all you need to create an instance of your contract using contract name.

```js
const contract = new Contract('sample-project');

// To deploy your contract
const deploy_response = await contract.deploy(account);

// To initialize your contract
await contract.instantiate({"count": 102}, "deploy test", account);
```

Note: You can check out your contract information in `deploy_response`.

# Interacting with contracts

`wasmkit` will load functions using schema, you can call contract functions using `contract.tx`(to execute transactions) and `contract.query`(to query from contract).

```js
// To interact with your contract
// Execute contract function
await contract.tx.increment(account);

// View count in contract
await contract.query.get_count();
```
