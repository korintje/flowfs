<template>
  <component :is="getIcon()"></component>
</template>

<script setup lang="ts">
import IconBitcoin from "@/assets/cryptoicons/btc.svg?component"
import IconBitcoinCash from "@/assets/cryptoicons/bch.svg?component"
import IconDash from "@/assets/cryptoicons/dash.svg?component"
import IconDogecoin from "@/assets/cryptoicons/doge.svg?component"
import IconEthereum from "@/assets/forkawesome/ethereum.svg?component"
import IconLitecoin from "@/assets/cryptoicons/ltc.svg?component"
import IconMonero from "@/assets/cryptoicons/xmr.svg?component"
import IconZcash from "@/assets/cryptoicons/zec.svg?component"
import IconBitcoinLightning from "@/assets/extra-icons/lightning.svg?component"

import { CurrencyCode } from "@/utils/cryptocurrencies"

const props = defineProps<{
  code: string,
}>()

function getIcon() {
  switch (props.code) {
    case CurrencyCode.Bitcoin:
      return IconBitcoin
    case CurrencyCode.BitcoinCash:
      return IconBitcoinCash
    case CurrencyCode.Dash:
      return IconDash
    case CurrencyCode.Dogecoin:
      return IconDogecoin
    case CurrencyCode.Ethereum:
      return IconEthereum
    case CurrencyCode.Litecoin:
      return IconLitecoin
    case CurrencyCode.Monero:
      return IconMonero
    case CurrencyCode.Zcash:
      return IconZcash
    case CurrencyCode.BitcoinLightning:
      return IconBitcoinLightning
    default:
      throw Error("unexpected currency code")
  }
}
</script>
