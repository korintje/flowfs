export function createDidFromEthereumAddress(address: string): string {
  return `did:pkh:eip155:1:${address.toLowerCase()}`
}
