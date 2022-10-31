const nearAPI = require("near-api-js");
const {
  utils: {
    format: { parseNearAmount },
  },
} = nearAPI;
const getConfig = require("./config");
const {
  networkId,
  stakingContractName,
  nftContractName,
  ownerAccountName,
  ftContractName,
  contractMethods,
  gas,
  gas_max,
  nodeUrl,
  walletUrl,
  explorerUrl,
} = getConfig("testnet");

const keyStore1 = new nearAPI.keyStores.InMemoryKeyStore();
const keyStore2 = new nearAPI.keyStores.InMemoryKeyStore();
const keyStore3 = new nearAPI.keyStores.InMemoryKeyStore();
const keyStore4 = new nearAPI.keyStores.InMemoryKeyStore();
const PRIVATE_KEY1 =
  "ed25519:9nYNwsP7mYqMLRsouSLaqKCBZTFcs8R34CVWgHNqoP351VVqsRmdPvKax8XCqWcKnGNsy45AYuofw6UsMPEJfdE"; //vier1near
const PRIVATE_KEY2 =
  "ed25519:2F9g25e6eXCXtKLtHr7SQddBrqdqXDJBLqHuWKy7SNQ9hqCbWP9FGcNpqaryyxiUUfGKQUKz4Uch743WMKkDU1M2"; //viernear
const PRIVATE_KEY3 =
  "ed25519:35XQmYA5Ps3D4jdxLTTZmKwWqRThi4R98mcM3eswbGJanYpnPvZ3esAfUDKptM1YNwizZwEMJE1Gct2sUxG5EmHa"; //oplec.testnet
const PRIVATE_KEY4 =
  "ed25519:W7gW6CQT3qJ2yTQLkwdXCbGX15MZ8uEUiApF3m2BmJWDoG38duS2op99KPGuvWVHByFu8xv7vebVw4saN17xTyU"; // marblestaking.testnet
const keyPair1 = nearAPI.KeyPair.fromString(PRIVATE_KEY1);
const keyPair2 = nearAPI.KeyPair.fromString(PRIVATE_KEY2);
const keyPair3 = nearAPI.KeyPair.fromString(PRIVATE_KEY3);
const keyPair4 = nearAPI.KeyPair.fromString(PRIVATE_KEY4);
keyStore1.setKey("testnet", "vier1near.testnet", keyPair1);
keyStore2.setKey("testnet", "viernear.testnet", keyPair2);
keyStore3.setKey("testnet", "oplec.testnet", keyPair3);
keyStore4.setKey("testnet", "marblestaking.testnet", keyPair4);

const near1 = new nearAPI.Near({
  deps: {
    keyStore: keyStore1,
  },
  networkId: networkId,
  keyStore: keyStore1,
  nodeUrl: nodeUrl,
  walletUrl: walletUrl,
});
const near2 = new nearAPI.Near({
  deps: {
    keyStore: keyStore2,
  },
  networkId: networkId,
  keyStore: keyStore2,
  nodeUrl: nodeUrl,
  walletUrl: walletUrl,
});
const near3 = new nearAPI.Near({
  deps: {
    keyStore: keyStore3,
  },
  networkId: networkId,
  keyStore: keyStore3,
  nodeUrl: nodeUrl,
  walletUrl: walletUrl,
});
const near4 = new nearAPI.Near({
  deps: {
    keyStore: keyStore4,
  },
  networkId: networkId,
  keyStore: keyStore4,
  nodeUrl: nodeUrl,
  walletUrl: walletUrl,
});

const stakingContractAccount = new nearAPI.Account(
  near4.connection,
  stakingContractName
);
const ownerAccount = new nearAPI.Account(near4.connection, ownerAccountName);

// Use your own account that logged in on NEAR CLI
const tokenOwnerAccount = new nearAPI.Account(
  near2.connection,
  "viernear.testnet"
);
const bidderAccount = new nearAPI.Account(
  near1.connection,
  "vier1near.testnet"
);
const bidderAccountName = "vier1near.testnet";
const bidderAccount2 = new nearAPI.Account(near3.connection, "oplec.testnet");

const stakingContract = new nearAPI.Contract(
  ownerAccount,
  stakingContractName,
  {
    viewMethods: [
      "get_owner",
      "get_config",
      "get_enable_status",
      "get_total_amount",
      "storage_minimum_balance",
      "storage_balance_of",
      "get_stake_info",
      "get_all_stake_info",
    ],
    changeMethods: [
      "new",
      "storage_deposit",
      "storage_withdraw",
      "update_owner",
      "update_enable",
      "claim_rewards",
    ],
  }
);

const nftContract = new nearAPI.Contract(ownerAccount, nftContractName, {
  viewMethods: ["nft_token", "nft_get_series"],
  changeMethods: [
    "nft_transfer",
    "nft_transfer_call",
    "new_default_meta",
    "nft_create_series",
    "nft_mint",
  ],
});
const ftContract = new nearAPI.Contract(ownerAccount, ftContractName, {
  viewMethods: ["ft_balance_of"],
  changeMethods: ["ft_transfer", "ft_transfer_call", "storage_deposit", "new"],
});

module.exports = {
  near1,
  gas,
  gas_max,
  keyStore1,
  stakingContractName,
  nftContractName,
  ownerAccountName,
  stakingContract,
  nftContract,
  ownerAccount,
  contractMethods,
  tokenOwnerAccount,
  bidderAccount,
  bidderAccountName,
  bidderAccount2,
  explorerUrl,
  ftContract,
  ftContractName,
  stakingContractAccount,
};
