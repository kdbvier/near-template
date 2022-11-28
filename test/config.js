const stakingContractName = "staking2.marblestaking.testnet";
const nftContractName = "nft.marblestaking.testnet";
const ownerAccountName = "marblestaking.testnet";
const ftContractName = "ft.marblestaking.testnet";

module.exports = function getConfig(network = "testnet") {
  let config = {
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
    stakingContractName: stakingContractName,
    nftContractName: nftContractName,
    ftContractName: ftContractName,
    ownerAccountName: ownerAccountName,
  };

  switch (network) {
    case "testnet":
      config = {
        ...config,
        GAS: "300000000000000",
        gas: "300000000000000",
        gas_max: "300000000000000",
      };
  }

  return config;
};
