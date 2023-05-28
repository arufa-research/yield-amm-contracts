import { getAccountByName } from "@arufa/wasmkit";

import externalContracts from "./external_contracts.json";
import { MarsAdapterContract } from "../artifacts/typescript_schema/MarsAdapterContract";
// import { TokenfactoryIssuerContract } from "../artifacts/typescript_schema/TokenfactoryIssuerContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  // const tokenfactory_issuer = new TokenfactoryIssuerContract();
  const mars_adapter = new MarsAdapterContract();
  // await tokenfactory_issuer.setupClient();
  await mars_adapter.setupClient();

  const underlyingDenom = "uosmo";
  const ybtDenom = "osmomars";  // fullDenom: factory/<contract_address>/<subdenom>

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
        {
          denom: underlyingDenom,
          amount: "1000000", // 1 OSMO
        },
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
}
