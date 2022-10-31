const nearAPI = require("near-api-js");
const assert = require("assert");
const testUtils = require("./test-utils");
const { NONAME } = require("dns");

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
  bidderAccountName,
} = testUtils;

const ftTokenAccount = "hera.cmdev0.testnet";

const stakingInit = async () => {
  try {
    await stakingContract.new({
      args: {
        owner_id: ownerAccountName,
        nft_address: nftContractName,
        ft_address: ftContractName,
        daily_reward: 1000,
        interval: 3600,
        lock_time: 3600,
      },
    });
  } catch (error) {
    console.log("staking init Error: ", error);
  }
};
const nftInit = async () => {
  try {
    await nftContract.new_default_meta({
      args: {
        owner_id: ownerAccountName,
        treasury_id: ownerAccountName,
      },
    });
  } catch (error) {
    console.log("nft init Error: ", error);
  }
};
const ftInit = async () => {
  try {
    await ftContract.new({
      args: {},
    });
  } catch (error) {
    console.log("init Error: ", error);
  }
};

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
    //   "get_owner",
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
    //   gas
    // );
    // const ft_amount = await ownerAccount.functionCall({
    //   contractId: stakingContractName,
    //   methodName: "get_total_amount",
    //   args: {},
    //   gas: gas,
    // });
    // console.log("ft_amount: ", ft_amount);
  } catch (err) {
    console.log("errrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr: ", err);
    throw err;
  }
};

const ft_deposit_in_staking = async () => {
  // const ft_balance = await ownerAccount.viewFunction(
  //   ftContractName,
  //   "ft_balance_of",
  //   {
  //     account_id: "vier1near.testnet",
  //   }
  // );
  // console.log("ft balance before staking: ", ft_balance);
  // await bidderAccount.functionCall({
  //   contractId: ftContractName,
  //   methodName: "ft_transfer",
  //   args: {
  //     receiver_id: stakingContractName,
  //     amount: "10000000000000000",
  //   },
  //   gas: gas,
  //   attachedDeposit: "1",
  // });
  const after_ft_balance = await ownerAccount.viewFunction(
    ftContractName,
    "ft_balance_of",
    {
      account_id: stakingContractName,
    }
  );
  console.log("ft balance before staking: ", after_ft_balance);
  const storage_deposit = await ownerAccount.functionCall({
    contractId: ftContractName,
    methodName: "storage_deposit",
    args: {
      account_id: ownerAccountName,
    },
    gas: gas,
    attachedDeposit: "10000000000000000000000",
  });
  // await ownerAccount.functionCall({
  //   contractId: ftContractName,
  //   methodName: "mint",
  //   args: {
  //     account_id: "vier4near.testnet",
  //     amount: "10000000000000000",
  //   },
  //   gas: gas,
  // });
  // const after_ft_balance = await ownerAccount.viewFunction(
  //   ftContractName,
  //   "ft_balance_of",
  //   {
  //     account_id: "vier4near.testnet",
  //   }
  // );
  // console.log("ft balance before staking: ", after_ft_balance);
};

const nft_staking = async () => {
  // const formattedParams = {
  //   token_metadata: {
  //     title: "Dark",
  //     media: "bafybeifdbvb6yzajogbe4dbn3bgxoli3sp7ol7upfmu2givpvbwufydthu",
  //     reference: "bafybeifvzitvju4ftwnkf7w7yakz7i5colcey223uk2ui4t5z3ss7l2od4",
  //     copies: 100,
  //   },
  //   price: null,
  //   royalty: {
  //     "viernear.testnet": 1000,
  //   },
  //   creator_id: "viernear.testnet",
  // };

  // const ret = await nftContract.nft_create_series(
  //   formattedParams,
  //   300000000000000, //	attached GAS
  //   "8540000000000000000000"
  // );

  // const ret = await ownerAccount.functionCall({
  //   contractId: nftContractName,
  //   methodName: "nft_create_series",
  //   args: {
  //     token_metadata: {
  //       title: "Dark",
  //       media: "bafybeifdbvb6yzajogbe4dbn3bgxoli3sp7ol7upfmu2givpvbwufydthu",
  //       reference:
  //         "bafybeifvzitvju4ftwnkf7w7yakz7i5colcey223uk2ui4t5z3ss7l2od4",
  //       copies: 100,
  //     },
  //     price: null,
  //     royalty: {
  //       "viernear.testnet": 1000,
  //     },
  //     creator_id: "viernear.testnet",
  //   },
  //   gas: gas,
  //   attachedDeposit: "8540000000000000000000",
  // });

  // const nft_series = await nftContract.nft_get_series();
  // console.log("nft_series: ", nft_series);

  // const result = await ownerAccount.functionCall({
  //   contractId: nftContractName,
  //   methodName: "nft_mint",
  //   args: {
  //     token_series_id: "1",
  //     // receiver_id: ownerAccountName,
  //     receiver_id: bidderAccountName,
  //   },
  //   gas: gas,
  //   attachedDeposit: "8540000000000000000000",
  // });

  // const minted_nft = await ownerAccount.viewFunction(
  //   nftContractName,
  //   "nft_token",
  //   {
  //     token_id: "1:3",
  //   }
  // );
  // console.log("minted_nft: ", minted_nft);

  // // await ownerAccount.functionCall({
  // await bidderAccount.functionCall({
  //   contractId: nftContractName,
  //   methodName: "nft_transfer_call",
  //   args: {
  //     receiver_id: stakingContractName,
  //     token_id: "1:4",
  //     msg: JSON.stringify({}),
  //   },
  //   gas: gas,
  //   attachedDeposit: "1",
  // });

  const staked_nft = await ownerAccount.viewFunction(
    nftContractName,
    "nft_token",
    {
      token_id: "1:4",
    }
  );
  console.log("staked_nft: ", staked_nft);

  // const pre_balance = await ownerAccount.viewFunction(
  //   ftContractName,
  //   "ft_balance_of",
  //   {
  //     account_id: bidderAccountName,
  //   }
  // );
  // console.log("pre_balance: ", pre_balance);

  // const config = await ownerAccount.viewFunction(
  //   stakingContractName,
  //   "get_all_stake_info",
  //   {}
  // );
  // console.log("stake_info: ", config);

  // const unstake_config = await ownerAccount.viewFunction(
  //   stakingContractName,
  //   "get_stake_info",
  //   {
  //     owner: ownerAccountName,
  //     // owner: bidderAccountName,
  //   }
  // );
  // console.log("unstake_info: ", unstake_config);

  // const claim = await bidderAccount.functionCall({
  //   contractId: stakingContractName,
  //   methodName: "claim_rewards",
  //   args: {},
  //   gas: gas,
  //   attachedDeposit: "1",
  // });

  // console.log("claim: ", claim);

  // const claim_config = await ownerAccount.viewFunction(
  //   stakingContractName,
  //   "get_stake_info",
  //   {
  //     owner: bidderAccountName,
  //     // owner: bidderAccountName,
  //   }
  // );
  // console.log("claim_config: ", claim_config);
  // await bidderAccount.functionCall({
  //   contractId: stakingContractName,
  //   methodName: "create_unstake",
  //   args: {},
  //   gas: gas,
  //   attachedDeposit: "1",
  // });

  // await bidderAccount.functionCall({
  //   contractId: stakingContractName,
  //   methodName: "fetch_unstake",
  //   args: {},
  //   gas: gas,
  // });
  // const after_claim_config = await ownerAccount.viewFunction(
  //   stakingContractName,
  //   "get_stake_info",
  //   {
  //     // owner: ownerAccountName,
  //     owner: bidderAccountName,
  //   }
  // );
  // console.log("after_claim_config: ", after_claim_config);
  // const after_balance = await ownerAccount.viewFunction(
  //   ftContractName,
  //   "ft_balance_of",
  //   {
  //     account_id: bidderAccountName,
  //   }
  // );
  // console.log("after_balance: ", after_balance);
};

// getConfig();
// ft_deposit_in_staking();
nft_staking();

// stakingInit();
// nftInit();
// ftInit();
