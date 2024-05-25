export enum CurrencyCode {
  Bitcoin = "BTC",
  BitcoinCash = "BCH",
  Dash = "DASH",
  Dogecoin = "DOGE",
  Ethereum = "ETH",
  Litecoin = "LTC",
  Monero = "XMR",
  Zcash = "ZEC",

  BitcoinLightning = "LN",
}

// Currency code; currency name; payment URI scheme
const CRYPTOCURRENCIES: [CurrencyCode, string, string][] = [
  // https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki
  [CurrencyCode.Bitcoin, "Bitcoin", "bitcoin"],

  // https://bitcoincashstandards.org/
  [CurrencyCode.BitcoinCash, "Bitcoin Cash", "bitcoincash"],

  [CurrencyCode.Dash, "Dash", "dash"],
  [CurrencyCode.Dogecoin, "Dogecoin", "dogecoin"],

  // https://eips.ethereum.org/EIPS/eip-681
  // Not supported by MetaMask https://github.com/MetaMask/metamask-extension/issues/5125
  [CurrencyCode.Ethereum, "Ethereum", "ethereum"],

  // https://electrum-ltc.org/litecoin_URIs.html
  [CurrencyCode.Litecoin, "Litecoin", "litecoin"],

  // https://github.com/monero-project/monero/wiki/URI-Formatting
  [CurrencyCode.Monero, "Monero", "monero"],

  // https://zips.z.cash/zip-0321
  [CurrencyCode.Zcash, "Zcash", "zcash"],
]

export interface Currency {
  code: CurrencyCode,
  name: string,
}

export const MONERO: Currency = { code: CurrencyCode.Monero, name: "Monero" }
export const ETHEREUM: Currency = { code: CurrencyCode.Ethereum, name: "Ethereum" }

export function getCurrencyByLabel(label: string): Currency | null {
  const currency = CRYPTOCURRENCIES.find(([code]) => {
    return `$${code}` === label.toUpperCase()
  })
  if (currency) {
    return { code: currency[0], name: currency[1] }
  }
  if (["⚡", "lightning address", "lud16"].includes(label.toLowerCase())) {
    return { code: CurrencyCode.BitcoinLightning, name: "Bitcoin Lightning" }
  }
  return null
}

export function isEthereumChain(chainId: string): boolean {
  return chainId.startsWith("eip155")
}

export function isMoneroChain(chainId: string): boolean {
  return chainId.startsWith("monero")
}
