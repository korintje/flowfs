<template>
  <div class="subscription">
    <div class="participants">
      <component
        class="profile-card"
        :is="sender.id ? 'router-link' : 'div'"
        :to="getActorLocation('profile', sender)"
      >
        <avatar :profile="sender"></avatar>
        <profile-display-name :profile="sender"></profile-display-name>
        <div class="wallet-address">{{ walletAddress ? walletAddress.toLowerCase() : '?' }}</div>
      </component>
      <div class="separator">
        <icon-arrow-right></icon-arrow-right>
      </div>
      <router-link
        class="profile-card"
        :to="getActorLocation('profile', recipient)"
      >
        <avatar :profile="recipient"></avatar>
        <profile-display-name :profile="recipient"></profile-display-name>
        <div class="wallet-address">{{ recipientEthereumAddress }}</div>
      </router-link>
    </div>
    <div class="connect-wallet" v-if="canConnectWallet()">
      <button class="btn primary" @click="connectWallet()">Connect wallet</button>
    </div>
    <div class="wallet-error" v-if="walletError">
      {{ walletError }}
    </div>
    <div class="info" v-if="subscriptionsEnabled !== null">
      <template v-if="subscriptionConfig">
        <div class="price">
          {{ subscriptionConfig.pricePerMonth }} {{ subscriptionConfig.tokenSymbol }}
          <span class="price-subtext">per month</span>
        </div>
        <div class="status" v-if="subscriptionState">
          <template v-if="subscriptionState.senderBalance.isZero()">
            You are not subscribed yet
          </template>
          <template v-else>
            <div>
              Your balance is
              {{ subscriptionConfig.formatAmount(subscriptionState.senderBalance) }}
              {{ subscriptionConfig.tokenSymbol }}
            </div>
            <div>
              Subscription expires
              {{ subscriptionConfig.getExpirationDate(subscriptionState.senderBalance).toLocaleString() }}
            </div>
          </template>
        </div>
      </template>
      <template v-else>
        Subscription is not available.
      </template>
    </div>
    <form class="payment" v-if="canSubscribe()">
      <div class="duration">
        <label for="duration">Duration</label>
        <input type="number" id="duration" v-model="paymentDuration" min="1">
        <span>months</span>
      </div>
      <div v-if="subscriptionConfig !== null">
        <div class="payment-amount">
          <label>Amount</label>
          <div>{{ getPaymentAmount() }} {{ subscriptionConfig.tokenSymbol }}</div>
        </div>
        <div
          v-if="tokenBalance !== null"
          class="token-balance"
          :class="{ error: !canPay() }"
        >
          <label>You have</label>
          <output :class="{ loading: isTokenBalanceLoading }">
            {{ subscriptionConfig.formatAmount(tokenBalance) }}
          </output>
          <span>{{ subscriptionConfig.tokenSymbol }}</span>
          <button @click.prevent="refreshTokenBalance()">
            <icon-refresh></icon-refresh>
          </button>
        </div>
      </div>
      <div class="button-row">
        <button
          type="submit"
          class="btn primary"
          :disabled="!canPay()"
          @click.prevent="onMakeSubscriptionPayment()"
        >
          <template v-if="!subscriptionState || subscriptionState.senderBalance.isZero()">
            Pay
          </template>
          <template v-else>Extend</template>
        </button>
        <button
          v-if="isBalancePositive()"
          class="btn secondary"
          :disabled="!canCancel()"
          @click.prevent="onCancelSubscription()"
        >
          Cancel
        </button>
      </div>
    </form>
    <loader v-if="isLoading"></loader>
  </div>
</template>

<script setup lang="ts">
import { BigNumber, FixedNumber } from "ethers"
import { onMounted, watch } from "vue"
import { $, $$, $computed, $ref } from "vue/macros"

import { searchProfilesByEthereumAddress } from "@/api/search"
import { defaultProfile, Profile, ProfileWrapper } from "@/api/users"
import {
  cancelSubscription,
  getSubscriptionConfig,
  getSubscriptionState,
  getTokenBalance,
  makeSubscriptionPayment,
  SubscriptionConfig,
  SubscriptionState,
} from "@/api/subscriptions-ethereum"
import IconArrowRight from "@/assets/feather/arrow-right.svg?component"
import IconRefresh from "@/assets/feather/refresh-ccw.svg?component"
import Avatar from "@/components/Avatar.vue"
import Loader from "@/components/Loader.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"
import { useWallet } from "@/composables/wallet"
import { ethereumAddressMatch } from "@/utils/ethereum"

const props = defineProps<{
  profile: Profile,
}>()

const { getActorLocation } = useActorHandle()
const { currentUser } = $(useCurrentUser())
const { getBlockchainInfo } = $(useInstanceInfo())
const { connectWallet: connectEthereumWallet } = useWallet()
const recipient = new ProfileWrapper(props.profile)
const recipientEthereumAddress = recipient.getVerifiedEthereumAddress()
let sender = $ref(new ProfileWrapper(currentUser || defaultProfile({ display_name: "You" })))
let { walletAddress, walletError, getSigner } = $(useWallet())
let subscriptionsEnabled = $ref<boolean | null>(null)
let subscriptionConfig = $ref<SubscriptionConfig | null>(null)
let subscriptionState = $ref<SubscriptionState | null>(null)
let tokenBalance = $ref<BigNumber | null>(null)
const paymentDuration = $ref<number>(1)

let isLoading = $ref(false)
let isTokenBalanceLoading = $ref(false)

onMounted(() => {
  if (walletAddress && !walletError) {
    // Load info immediately if wallet is already connected
    checkSubscription()
  }
})

const blockchain = $computed(() => getBlockchainInfo())

function canConnectWallet(): boolean {
  return (
    Boolean(blockchain?.contract_address) &&
    Boolean(blockchain?.features.subscriptions) &&
    // Only profiles with verified address can have subscription
    recipientEthereumAddress !== null &&
    walletAddress === null
  )
}

function reset() {
  subscriptionsEnabled = null
  subscriptionConfig = null
  subscriptionState = null
  tokenBalance = null
}

async function connectWallet() {
  await connectEthereumWallet()
  if (!walletError) {
    checkSubscription()
  }
}

watch($$(walletAddress), (newValue) => {
  if (newValue === null) {
    reset()
  }
})

async function checkSubscription() {
  if (
    !blockchain?.contract_address ||
    !recipientEthereumAddress ||
    !walletAddress
  ) {
    return
  }
  if (ethereumAddressMatch(walletAddress, recipientEthereumAddress)) {
    walletError = "Incorrect wallet address"
    return
  }
  // Update sender info
  const profiles = await searchProfilesByEthereumAddress(walletAddress)
  if (profiles.length === 1) {
    sender = new ProfileWrapper(profiles[0])
  } else {
    console.warn("can't find profile by wallet address")
  }
  const signer = getSigner()
  isLoading = true
  subscriptionConfig = await getSubscriptionConfig(
    blockchain.contract_address,
    signer,
    recipientEthereumAddress,
  )
  if (subscriptionConfig !== null) {
    subscriptionsEnabled = true
  } else {
    subscriptionsEnabled = false
    isLoading = false
    return
  }
  subscriptionState = await getSubscriptionState(
    blockchain.contract_address,
    signer,
    walletAddress,
    recipientEthereumAddress,
  )
  tokenBalance = await getTokenBalance(
    signer,
    subscriptionConfig.tokenAddress,
  )
  isLoading = false
}

function canSubscribe(): boolean {
  return (
    sender.id !== recipient.id &&
    subscriptionsEnabled === true
  )
}

function getPaymentAmount(): FixedNumber {
  if (!subscriptionConfig) {
    return FixedNumber.from(0)
  }
  const amount = subscriptionConfig.pricePerMonthInt.mul(paymentDuration)
  return subscriptionConfig.formatAmount(amount)
}

function canPay(): boolean {
  if (!subscriptionConfig || !tokenBalance || isLoading) {
    return false
  }
  const amount = subscriptionConfig.pricePerMonthInt.mul(paymentDuration)
  return amount.lte(tokenBalance)
}

async function refreshTokenBalance() {
  if (!subscriptionConfig) {
    return
  }
  const signer = getSigner()
  isTokenBalanceLoading = true
  tokenBalance = await getTokenBalance(signer, subscriptionConfig.tokenAddress)
  isTokenBalanceLoading = false
}

async function onMakeSubscriptionPayment() {
  if (
    !blockchain?.contract_address ||
    !recipientEthereumAddress ||
    !walletAddress ||
    !subscriptionConfig ||
    !subscriptionState
  ) {
    return
  }
  const signer = getSigner()
  const amount = subscriptionConfig.pricePerMonthInt.mul(paymentDuration)
  isLoading = true
  let transaction
  try {
    transaction = await makeSubscriptionPayment(
      blockchain.contract_address,
      signer,
      recipientEthereumAddress,
      amount,
    )
  } catch (error) {
    console.error(error)
    isLoading = false
    return
  }
  await transaction.wait()
  // Wait for sender balance update
  // because JSON-RPC API can return outdated info on the first call
  let newSubscriptionState
  while (!newSubscriptionState || subscriptionState.senderBalance === newSubscriptionState.senderBalance) {
    newSubscriptionState = await getSubscriptionState(
      blockchain.contract_address,
      signer,
      walletAddress,
      recipientEthereumAddress,
    )
  }
  subscriptionState = newSubscriptionState
  tokenBalance = await getTokenBalance(signer, subscriptionConfig.tokenAddress)
  isLoading = false
}

function isBalancePositive(): boolean {
  return (
    subscriptionState !== null &&
    !subscriptionState.senderBalance.isZero()
  )
}

function canCancel(): boolean {
  return isBalancePositive() && !isLoading
}

async function onCancelSubscription() {
  if (
    !blockchain?.contract_address ||
    !recipientEthereumAddress ||
    !walletAddress ||
    !subscriptionConfig
  ) {
    return
  }
  const signer = getSigner()
  isLoading = true
  let transaction
  try {
    transaction = await cancelSubscription(
      blockchain.contract_address,
      signer,
      recipientEthereumAddress,
    )
  } catch (error) {
    console.error(error)
    isLoading = false
    return
  }
  await transaction.wait()
  subscriptionState = await getSubscriptionState(
    blockchain.contract_address,
    signer,
    walletAddress,
    recipientEthereumAddress,
  )
  tokenBalance = await getTokenBalance(signer, subscriptionConfig.tokenAddress)
  isLoading = false
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.subscription {
  display: flex;
  flex-direction: column;
  gap: $block-outer-padding;
  text-align: center;
}

.participants {
  $avatar-size: 60px;

  align-items: center;
  display: flex;
  gap: $block-inner-padding;

  .profile-card {
    background-color: var(--block-background-color);
    border-radius: $block-border-radius;
    display: flex;
    flex-basis: 50%;
    flex-direction: column;
    gap: calc($block-inner-padding / 2);
    min-width: 0;
    padding: $block-inner-padding;
  }

  .separator svg {
    height: $icon-size;
    min-width: $icon-size;
    object-fit: contain;
    stroke: var(--text-color);
    width: $icon-size;
  }

  .avatar {
    height: $avatar-size;
    margin: 0 auto;
    width: $avatar-size;
  }

  .display-name {
    font-size: 16px;
  }

  .wallet-address {
    font-family: monospace;
    overflow: hidden;
    text-align: center;
    text-overflow: ellipsis;
  }
}

.wallet-error {
  color: $error-color;
}

.info {
  background-color: var(--block-background-color);
  border-radius: $block-border-radius;
  display: flex;
  flex-direction: column;
  gap: calc($block-inner-padding / 2);
  padding: $block-inner-padding;

  .price {
    font-size: 16px;
    font-weight: bold;
  }

  .price-subtext {
    font-size: $text-font-size;
  }

  .status {
    color: var(--secondary-text-color);
  }
}

.payment {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;

  .duration,
  .payment-amount,
  .token-balance {
    align-items: center;
    display: flex;
    gap: $input-padding;
    justify-content: center;
  }

  .duration {
    font-size: 16px;

    label {
      font-weight: bold;
    }

    input {
      width: 100px;
    }
  }

  .payment-amount {
    font-size: 16px;
    font-weight: bold;
    margin-bottom: calc($input-padding / 2);
  }

  .token-balance {
    color: var(--secondary-text-color);

    output.loading {
      opacity: 0.5;
    }

    svg {
      height: 1em;
      min-width: 1em;
      stroke: var(--secondary-text-color);
    }

    &.error {
      color: var(--text-color);
    }
  }

  .button-row {
    display: flex;
    gap: $block-inner-padding;
  }
}

.loader {
  margin: 0 auto;
}
</style>
