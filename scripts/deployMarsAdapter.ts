import { getAccountByName } from "@arufa/wasmkit";

import { MarsAdapterContract } from "../artifacts/typescript_schema/MarsAdapterContract";
import externalContracts from "./external_contracts.json";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const mars_adapter = new MarsAdapterContract();
  await mars_adapter.setupClient();

  const adapter_deploy_response = await mars_adapter.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uosmo" }],
      gas: "12000000",
    }
  );
  console.log(adapter_deploy_response);

  const customFees = { // custom fees
    amount: [{ amount: "750000", denom: "uosmo" }],
    gas: "4000000",
  };
  const contract_info = await mars_adapter.instantiate(
    {"red_bank": externalContracts.red_bank.contract_addr},
    `mars_adapter ${runTs}`,
    contract_owner,
    undefined,  // transferAmount
    customFees,
  );
  console.log(contract_info);

  const deposit_response = await mars_adapter.totalDeposit();
  console.log(deposit_response);

  const state_response = await mars_adapter.state();
  console.log(state_response);

  const config_response = await mars_adapter.config();
  console.log(config_response);

  // const ex_response = await mars_adapter.increment(
  //   {account: contract_owner, customFees: customFees}
  // );
  // console.log(ex_response);
}
