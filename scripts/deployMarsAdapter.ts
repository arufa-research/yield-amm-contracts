import { getAccountByName } from "@arufa/wasmkit";

import externalContracts from "./external_contracts.json";
import { MarsAdapterContract } from "../artifacts/typescript_schema/MarsAdapterContract";
import { SplitterContract } from "../artifacts/typescript_schema/SplitterContract";
import { MarketContract } from "../artifacts/typescript_schema/MarketContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const mars_adapter = new MarsAdapterContract();
  const splitter = new SplitterContract();
  const market = new MarketContract();
  await mars_adapter.setupClient();
  await splitter.setupClient();
  await market.setupClient();

  const underlyingDenom = "uosmo";
  const ybtDenom = "osmomars";  // fullDenom: factory/<contract_address>/ybtDenom
  const pDenom = "posmomars";  // fullDenom: factory/<contract_address>/pDenom
  const yDenom = "yosmomars";  // fullDenom: factory/<contract_address>/yDenom

  const customFees = { // custom fees
    amount: [{ amount: "750000", denom: "uosmo" }],
    gas: "4000000",
  };

  // DEPLOY MARS ADAPTER
  const adapter_deploy_response = await mars_adapter.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uosmo" }],
      gas: "12000000",
    }
  );
  console.log(adapter_deploy_response);

  const adapter_init_response = await mars_adapter.instantiate(
    {
      "red_bank": externalContracts.red_bank.contract_addr,
      "underlying_denom": underlyingDenom,
      "yield_bearing_denom": ybtDenom,
    },
    `mars_adapter ${runTs}`,
    contract_owner,
    [
      {
        denom: underlyingDenom,
        amount: "10000000", // 10 OSMO denom creation fee
      },
    ],
    customFees,
  );
  console.log(adapter_init_response);

  // const deposit_response_before = await mars_adapter.totalDeposit();
  // const state_response_before = await mars_adapter.state();
  // const config_response_before = await mars_adapter.config();

  // console.log(deposit_response_before);
  // console.log(state_response_before);
  // console.log(config_response_before);

  const do_deposit_response = await mars_adapter.deposit(
    {
      account: contract_owner,
      customFees: customFees,
      transferAmount: [
        {
          denom: underlyingDenom,
          amount: "1000000", // 1 OSMO
        },
      ],
    }
  );
  console.log(do_deposit_response);

  // const deposit_response_after = await mars_adapter.totalDeposit();
  // const state_response_after = await mars_adapter.state();
  // const config_response_after = await mars_adapter.config();
  
  // console.log(deposit_response_after);
  // console.log(state_response_after);
  // console.log(config_response_after);

  // const yb_balance = await osmo_mars_token.balance(
  //   { address: contract_owner.account.address }
  // );
  // console.log(yb_balance);

  // withdraw OSMO from contract
  const withdraw_reponse = await mars_adapter.withdraw(
    { 
      account: contract_owner,
      customFees: customFees,
      transferAmount: [
        {
          denom: `factory/${mars_adapter.contractAddress}/${ybtDenom}`,
          amount: "600000", // 0.6 OSMOmars
        },
      ],
    }
  );
  console.log(withdraw_reponse);

  // DEPLOY SPLITTER
  const splitter_deploy_response = await splitter.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uosmo" }],
      gas: "12000000",
    }
  );
  console.log(splitter_deploy_response);

  const splitter_init_response = await splitter.instantiate(
    {
      "red_bank": externalContracts.red_bank.contract_addr,
      "mars_adapter": mars_adapter.contractAddress,
      "underlying_denom": underlyingDenom,
      "epoch_period": 10, // in seconds
      "expiry_period": 360, // in seconds
      "yield_bearing_denom": ybtDenom,
      "principle_denom": pDenom,
      "yield_denom": yDenom,
    },
    `splitter ${runTs}`,
    contract_owner,
    [
      {
        denom: underlyingDenom,
        amount: "20000000", // 20 OSMO denom creation fee
      },
    ],
    customFees,
  );
  console.log(splitter_init_response);

  // TODO: deploy rewards contract
  // combination of native-stake and native-external-rewards

  const update_rewards_response = await splitter.updateRewardsContract(
    { account: contract_owner, customFees: customFees },
    { rewardsContract: externalContracts.red_bank.contract_addr },  // TODO: use the actual depoyed rewards_contract
  );
  console.log(update_rewards_response);

  // deposit ybToken and get pToken and yToken
  const yb_deposit_response = await splitter.deposit(
    {
      account: contract_owner,
      customFees: customFees,
      transferAmount: [{
        denom: `factory/${mars_adapter.contractAddress}/${ybtDenom}`,
        amount: "200000", // 0.2 OSMOmars
      }],
    },
  );
  console.log(JSON.stringify(yb_deposit_response, null, 2));

  // // withdraw ybToken by sending pToken and yToken
  // const yb_withdraw_response = await splitter.withdraw(
  //   {
  //     account: contract_owner,
  //     customFees: customFees,
  //     transferAmount: [
  //       {
  //         denom: `factory/${splitter.contractAddress}/${pDenom}`,
  //         amount: "110000", // 0.11 pOSMOmars
  //       },
  //       {
  //         denom: `factory/${splitter.contractAddress}/${yDenom}`,
  //         amount: "110000", // 0.11 yOSMOmars
  //       }
  //     ],
  //   },
  // );
  // console.log(JSON.stringify(yb_withdraw_response, null, 2));

  // DEPLOY MARKET (ybtDenom, pDenom) stableSwap pair with dynamic scaling factor
  const market_deploy_response = await market.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uosmo" }],
      gas: "12000000",
    }
  );
  console.log(market_deploy_response);

  const market_init_response = await market.instantiate(
    {
      "red_bank": externalContracts.red_bank.contract_addr,
      "mars_adapter": mars_adapter.contractAddress,
      "splitter": splitter.contractAddress,
      "underlying_denom": underlyingDenom,
      "yield_bearing_denom": ybtDenom,
      "principle_denom": pDenom,
      "yield_denom": yDenom,
    },
    `market ${runTs}`,
    contract_owner,
    [
      {
        denom: `factory/${splitter.contractAddress}/${pDenom}`,
        amount: "100000", // 0.1 pToken for initial liquidity
      },
      {
        denom: `factory/${mars_adapter.contractAddress}/${ybtDenom}`,
        amount: "100000", // 0.1 ybToken for initial liquidity
      },
      {
        denom: underlyingDenom,
        amount: "100000000", // 100 OSMO pair creation fee
      },
    ],
    customFees,
  );
  console.log(JSON.stringify(market_init_response, null, 2));

  const market_config = await market.config();
  const market_state = await market.state();

  console.log("market_config: ", market_config);
  console.log("market_state: ", market_state);
}
