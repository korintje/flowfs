import { BigNumber } from "@ethersproject/bignumber"

export function roundBigNumber(value: BigNumber, precision: number): BigNumber {
  const decimals = value.toString().length
  const divisor = BigNumber.from(10).pow(decimals - precision)
  const remainder = value.mod(divisor)
  const midpoint = BigNumber.from(10).pow(Math.max(decimals - precision - 1, 0)).mul(5)
  if (remainder.gte(midpoint)) {
    return value.div(divisor).add(1).mul(divisor)
  } else {
    return value.div(divisor).mul(divisor)
  }
}

function getPrecision(value: number): number {
  if (typeof value !== "number") {
    return 0
  }
  if (!isFinite(value)) {
    return 0
  }
  let precision = 0
  while (Math.round(value * Math.pow(10, precision)) / Math.pow(10, precision) !== value) {
    precision++
  }
  return precision
}

export function floatToBigNumber(value: number, decimals: number): BigNumber {
  const precision = getPrecision(value)
  const denominator = 10 ** precision
  const numerator = Math.round(value * denominator)
  return BigNumber.from(10).pow(decimals).mul(numerator).div(denominator)
}
