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
  bidderAccount,
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
    // const ft_amount = await ownerAccount.viewFunction(
    //   stakingContractName,
    //   "get_total_amount",
    //   {},

    // );
    const ft_amount = await ownerAccount.functionCall({
      contractId: stakingContractName,
      methodName: "get_total_amount",
      args: {},
      gas: gas,
    });
    console.log("ft_amount: ", ft_amount);
  } catch (err) {
    throw err;
  }
};

const nft_staking = async () => {
  const formattedParams = {
    token_metadata: {
      title: "Dark",
      media: "bafybeifdbvb6yzajogbe4dbn3bgxoli3sp7ol7upfmu2givpvbwufydthu",
      reference: "bafybeifvzitvju4ftwnkf7w7yakz7i5colcey223uk2ui4t5z3ss7l2od4",
      copies: 100,
    },
    price: null,
    royalty: {
      "viernear.testnet": 1000,
    },
    creator_id: "viernear.testnet",
  };

  // const ret = await nftContract.nft_create_series(
  //   formattedParams,
  //   300000000000000, //	attached GAS
  //   "8540000000000000000000"
  // );
  // const nft_series = await nftContract.nft_get_series();
  console.log("nft_series: ", nft_series);
  const minted_nft = await nftContract.nft_mint;
};

getConfig();
// nft_staking();

// stakingInit();
// nftInit();
// ftInit();
