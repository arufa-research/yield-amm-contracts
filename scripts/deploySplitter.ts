import { getAccountByName } from "@arufa/wasmkit";

import externalContracts from "./external_contracts.json";
import { MarsAdapterContract } from "../artifacts/typescript_schema/MarsAdapterContract";
import { YieldBearingTokenContract } from "../artifacts/typescript_schema/YieldBearingTokenContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const osmo_mars_token = new YieldBearingTokenContract();
  const mars_adapter = new MarsAdapterContract();
  await osmo_mars_token.setupClient();
  await mars_adapter.setupClient();

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
    },
    `mars_adapter ${runTs}`,
    contract_owner,
    undefined,  // transferAmount
    customFees,
  );
  console.log(adapter_init_response);

  // DEPLOY YIELD BEARING TOKEN
  const osmo_mars_deploy_response = await osmo_mars_token.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uosmo" }],
      gas: "12000000",
    }
  );
  console.log(osmo_mars_deploy_response);

  const osmo_mars_init_response = await osmo_mars_token.instantiate(
    {
      "decimals": 6,
      "initial_balances": [],
      "mint": {
        "minter": mars_adapter.contractAddress,
      },
      "name": "Yield bearing mars OSMO",
      "symbol": "OSMOmars",
    },
    `osmo_mars ${runTs}`,
    contract_owner,
    undefined,  // transferAmount
    customFees,
  );
  console.log(osmo_mars_init_response);

  const update_yb_response = await mars_adapter.updateYieldBearingToken(
    { account: contract_owner, customFees: customFees },
    { yieldBearingToken: osmo_mars_token.contractAddress }
  );
  console.log("update_yb_response: ", update_yb_response);

  const deposit_response_before = await mars_adapter.totalDeposit();
  console.log(deposit_response_before);

  const state_response_before = await mars_adapter.state();
  console.log(state_response_before);

  const config_response_before = await mars_adapter.config();
  console.log(config_response_before);

  const do_deposit_response = await mars_adapter.deposit(
    {
      account: contract_owner,
      customFees: customFees,
      transferAmount: [
        {denom: "uosmo", amount: "1000000"}, // 1 OSMO
      ],
    }
  );
  console.log(do_deposit_response);

  const deposit_response_after = await mars_adapter.totalDeposit();
  console.log(deposit_response_after);

  const state_response_after = await mars_adapter.state();
  console.log(state_response_after);

  const config_response_after = await mars_adapter.config();
  console.log(config_response_after);

  const yb_balance = await osmo_mars_token.balance(
    { address: contract_owner.account.address }
  );
  console.log(yb_balance);

  // withdraw OSMO from contract
  const withdraw_reponse = await osmo_mars_token.send(
    { account: contract_owner, customFees: customFees },
    {
      amount: "600000",  // 0.6 OSMOmars
      contract: mars_adapter.contractAddress,
      msg: Buffer.from(JSON.stringify({
        withdraw: {}
      })).toString("base64"),
    },
  );
  console.log(withdraw_reponse);
}
