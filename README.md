# FixedFi: Yield AMM contracts

<img src="./assets/FixedFiLogoColor.png" width=25% height=25%>

FixedFi is a DeFi protocol built using CosmWasm contracts that lets anyone access to fixed yield on their assets.

Split a yield bearing asset into principle token (pToken) and yield token (yToken) and yToken can be sold at a fixed price to fix the future yield and to get it now.

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

## Switching from variable yield to fixed yeild

1. A ybToken got from depositing Token to a yield generating DeFi protocol.
2. Deposit ybToken into FixedFi to get pToken and yToken for fixed duration (example: 6 months).
3. yToken represents the yield for the next 6 months which is variable.
4. pToken represents the underlying asset locked up for 6 months.
5. Sell the yToken at current market price through the AMM and fix the amount of yield for next 6 months.
6. After 6 months unlock the underlying token from pToken.

## Getting assets at discount (fixed APR with lock period)

1. Instead of buying the asset, buy the pToken of that asset.
2. pToken in the AMM is always at a dicount from underlying asset.
3. The discount is due to lockup period (example 6 months) and separation of yield for that period.
4. Directly buying pToken fixes the discount, hence fixing the yield on that asset for next 6 months.
5. After 6 months, unlock the asset from the pToken.
6. As time progresses, the price of pToken will steadily increase and will match the underlying at expiry.
7. The amount of discount is proportional to the expected yield which is seprated using yToken.

## Speculating on yield changes

1. yToken is selling at a fixed price at a time through the AMM.
2. The price represents the average yield over lock time that market expects.
3. If user belives that the yield can go up in future that buying the yToken can give a positive APR.
4. Buying more of yToken gives higher exposure to the future variable yield.

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
