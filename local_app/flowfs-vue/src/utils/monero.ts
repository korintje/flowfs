import { fromByteArray } from "base64-js"
import { DateTime } from "luxon"
import pako from "pako"

import { generateRandomString } from "./ethereum"

// https://github.com/monero-project/monero/wiki/URI-Formatting
export function createMoneroPaymentUri(
  address: string,
  amount?: number,
): string {
  let paymentUri = `monero:${address}`
  if (amount) {
    paymentUri += `?tx_amount=${amount}`
  }
  return paymentUri
}

export function createMoneroCaip122Message(
  address: string,
  domain: string,
  statement: string,
): string {
  const uri = window.location.origin
  const nonce = generateRandomString(16)
  const issuedAt = new Date().toISOString()
  const message = `${domain} wants you to sign in with your Monero account:
${address}

${statement}

URI: ${uri}
Version: 1
Nonce: ${nonce}
Issued At: ${issuedAt}`

  return message
}

// https://github.com/lukeprofits/Monero_Payment_Request_Standard
interface PaymentData {
  custom_label: string,
  sellers_wallet: string,
  currency: string,
  amount: string,
  payment_id: string,
  start_date: string,
  days_per_billing_cycle: number,
  number_of_payments: number,
  change_indicator_url: string,
}

export function createMoneroPaymentRequestV1(data: PaymentData): string {
  const dataJson = JSON.stringify(data, Object.keys(data).sort())
  const encoder = new TextEncoder()
  const dataBytes = encoder.encode(dataJson)
  const dataCompressed = pako.gzip(dataBytes, { level: 9, windowBits: 31 })
  const dataEncoded = fromByteArray(dataCompressed)
  return `monero-request:1:${dataEncoded}`
}

export function createMoneroPaymentRequest(
  label: string,
  amount: number,
  period: number,
  address: string,
  startDate: string,
): string {
  const startDate_ = DateTime.fromISO(startDate) // validate
  const data = {
    custom_label: label,
    sellers_wallet: address,
    currency: "XMR",
    amount: amount.toString(),
    payment_id: "0000000000000000",
    start_date: startDate_.toISO(),
    days_per_billing_cycle: period,
    number_of_payments: 0, // payments will recur until cancelled
    change_indicator_url: "",
  }
  return createMoneroPaymentRequestV1(data)
}
