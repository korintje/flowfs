import bs58 from "bs58"

export function hexToBytes(hexString: string): Uint8Array {
  const bytes = []
  for (let pos = 0; pos < hexString.length; pos += 2) {
    const byte = parseInt(hexString.substring(pos, pos + 2), 16)
    bytes.push(byte)
  }
  return new Uint8Array(bytes)
}

export function encodeMultibase(bytes: Uint8Array): string {
  const result = bs58.encode(bytes)
  return `z${result}`
}
