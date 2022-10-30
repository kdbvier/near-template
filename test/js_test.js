const nearAPI = require("near-api-js");
const assert = require("assert");
const testUtils = require("./test-utils");

const {
  gas,
  stakingContract,
  nftContractName,
  ownerAccountName,
  ftContractName,
  nftContract,
  ftContract,
  ownerAccount,
  stakingContractName,
} = testUtils;

const ftTokenAccount = "hera.cmdev0.testnet";

// const stakingInit = async () => {
//   try {
//     await stakingContract.new({
//       args: {
//         owner_id: ownerAccountName,
//         nft_address: nftContractName,
//         ft_address: ftContractName,
//         daily_reward: 1000,
//         interval: 3600,
//         lock_time: 3600,
//       },
//     });
//   } catch (error) {
//     console.log("staking init Error: ", error);
//   }
// };
// const nftInit = async () => {
//   try {
//     await nftContract.new_default_meta({
//       args: {
//         owner_id: ownerAccountName,
//         treasury_id: ownerAccountName,
//       },
//     });
//   } catch (error) {
//     console.log("nft init Error: ", error);
//   }
// };
// const ftInit = async () => {
//   try {
//     await ftContract.new({
//       args: {},
//     });
//   } catch (error) {
//     console.log("init Error: ", error);
//   }
// };

const getConfig = async () => {
  try {
    // const config = await ownerAccount.viewFunction(
    //   stakingContractName,
    //   "get_config",
    //   {}
    // );
    // console.log("staking-config: ", config);

    // await ownerAccount.functionCall({
    //   contractId: stakingContractName,
    //   methodName: "update_enable",
    //   args: {
    //     enabled: true,
    //   },
    //   gas: gas,
    //   attachedDeposit: "1",
    // });
    // const upadated_config = await ownerAccount.viewFunction(
    //   stakingContractName,
    //   "get_config",
    //   {}
    // );
    // console.log("staking-config: ", upadated_config);
    const ft_amount_in_ft_contract = await ownerAccount.viewFunction(
      ftContractName,
      "ft_balance_of",
      {
        account_id: stakingContractName,
      }
    );
    console.log("ft_amount_in_ft_contract: ", ft_amount_in_ft_contract);
    const ft_amount = await ownerAccount.viewFunction(
      stakingContractName,
      "get_total_amount",
      {}
    );
    console.log("ft_amount: ", ft_amount);
  } catch (err) {
    throw err;
  }
};

getConfig();

// stakingInit();
// nftInit();
// ftInit();
