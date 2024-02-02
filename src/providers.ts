export const providers = {
  gnosis_mainnet_archive: [
    "https://rpc.eu-central-2.gateway.fm/v4/gnosis/archival/mainnet?apiKey=tAb_6JUvPKeNV06gJ1YdctcMwTvQp4_F.2G6vFAJtwtvushb3"
  ],
  xdai_mainnet: [
    // FIXME: activate once upstream error handling works
    // `https://gnosis-provider.rpch.tech`
    `https://primary.gnosis-chain.rpc.hoprtech.net`
    // `https://secondary.gnosis-chain.rpc.hoprtech.net`
  ],
  eth_gnosis_mainnet_infura:
    "https://mainnet.infura.io/v3/7f8df481afe542f99fe5e01de8822202",
};
