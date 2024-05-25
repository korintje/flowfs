<template>
  <div class="subscription-settings">
    <div class="connect-wallet" v-if="canConnectWallet()">
      <button class="btn" @click="connectWallet()">Connect wallet</button>
    </div>
    <div class="wallet-error" v-if="walletError">
      {{ walletError }}
    </div>
    <div class="link-address" v-if="canVerifyEthereumAddress()">
      <button class="btn" @click="onVerifyEthereumAddress()">
        Link your address
      </button>
    </div>
    <div class="info" v-if="subscriptionsEnabled !== null">
      <template v-if="subscriptionConfig && subscriptionOption !== null">
        <span>Subscriptions are enabled</span>
        <div class="info-item">
          {{ subscriptionConfig.pricePerMonth }}
          {{ subscriptionConfig.tokenSymbol }}
          per month
        </div>
        <div class="info-item">
          {{ ensureCurrentUser().subscribers_count }} subscribers
        </div>
      </template>
      <template v-else>
        Subscriptions are not enabled
      </template>
    </div>
    <form class="setup" v-if="canEnableSubscriptions()">
      <div class="price">
        <label for="price">Price</label>
        <input type="number" id="price" v-model="subscriptionPrice" min="0.00">
        <span v-if="subscriptionToken">{{ subscriptionToken.symbol }} per month</span>
      </div>
      <button
        class="btn primary"
        :disabled="subscriptionPrice <= 0"
        @click.prevent="onEnableSubscriptions()"
      >
        Enable subscriptions
      </button>
    </form>
    <form class="withdraw" v-if="subscriptionConfig !== null">
      <h2>Subscribers</h2>
      <div
        v-for="subscription in subscriptions"
        class="subscriber"
        :class="{ expired: !isSubscriptionActive(subscription) }"
        :key="subscription.id"
        @click="onSubscriberSelected(subscription)"
      >
        <profile-list-item :profile="subscription.sender"></profile-list-item>
      </div>
      <input
        type="text"
        v-model="subscriberAddress"
        placeholder="Subscriber address"
      >
      <button
        class="btn"
        :disabled="!subscriberAddress"
        @click.prevent="onCheckSubsciptionState()"
      >
        Check
      </button>
      <button
        class="btn"
        v-if="subscriptionState !== null"
        @click.prevent="onWithdrawReceived()"
      >
        Withdraw
        {{ subscriptionConfig.formatAmount(subscriptionState.recipientBalance) }}
        {{ subscriptionConfig.tokenSymbol }}
      </button>
    </form>
    <loader v-if="isLoading"></loader>
  </div>
</template>

<script setup lang="ts">
import { onMounted, watch } from "vue"
import { $, $$, $computed, $ref } from "vue/macros"

import { ProfileWrapper } from "@/api/users"
import {
  getPricePerSec,
  getSubscriptionOptions,
  getReceivedSubscriptions,
  Subscription,
  SubscriptionOption,
} from "@/api/subscriptions-common"
import {
  configureSubscriptions,
  getSubscriptionAuthorization,
  getSubscriptionConfig,
  getSubscriptionState,
  getSubscriptionToken,
  onSubscriptionsEnabled,
  withdrawReceived,
  SubscriptionConfig,
  SubscriptionState,
  SubscriptionToken,
} from "@/api/subscriptions-ethereum"
import Loader from "@/components/Loader.vue"
import ProfileListItem from "@/components/ProfileListItem.vue"
import { useEthereumAddressVerification } from "@/composables/ethereum-address-verification"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"
import { useWallet } from "@/composables/wallet"
import { isPastDate } from "@/utils/dates"
import { ethereumAddressMatch } from "@/utils/ethereum"

const { ensureAuthToken, ensureCurrentUser, setCurrentUser } = $(useCurrentUser())
const { verifyEthereumAddress } = useEthereumAddressVerification()
const { getBlockchainInfo } = $(useInstanceInfo())
const { connectWallet: connectEthereumWallet, getSigner } = useWallet()
const subscriptionPrice = $ref<number>(1)

let { walletAddress, walletError } = $(useWallet())
let isLoading = $ref(false)
let subscriptionOption = $ref<SubscriptionOption | null>(null)
let subscriptionToken = $ref<SubscriptionToken | null>(null)
let subscriptionsEnabled = $ref<boolean | null>(null)
let subscriptionConfig = $ref<SubscriptionConfig | null>(null)
let subscriptionState = $ref<SubscriptionState | null>(null)
let subscriptions = $ref<Subscription[]>([])
let subscriberAddress = $ref<string | null>(null)

const blockchain = $computed(() => getBlockchainInfo())
const profile = $computed(() => new ProfileWrapper(ensureCurrentUser()))

onMounted(() => {
  if (walletAddress && !walletError) {
    // Load info immediately if wallet is already connected
    loadSubscriptionSettings()
  }
})

async function loadSubscriptionOption() {
  const subscriptionOptions = await getSubscriptionOptions(ensureAuthToken())
  subscriptionOption = subscriptionOptions.find((item) => {
    return item.type === "ethereum"
  }) || null
}

function canConnectWallet(): boolean {
  return (
    Boolean(blockchain?.contract_address) &&
    Boolean(blockchain?.features.subscriptions) &&
    walletAddress === null
  )
}

function reset() {
  subscriptionToken = null
  subscriptionsEnabled = null
  subscriptionConfig = null
  subscriptionState = null
  subscriberAddress = null
}

async function connectWallet() {
  await connectEthereumWallet()
  if (!walletError) {
    loadSubscriptionSettings()
  }
}

watch($$(walletAddress), (newValue) => {
  if (newValue === null) {
    reset()
  }
})

async function loadSubscriptionSettings() {
  const profileAddress = profile.getVerifiedEthereumAddress()
  if (
    !blockchain?.contract_address ||
    !profileAddress ||
    !walletAddress
  ) {
    return
  }
  if (!ethereumAddressMatch(walletAddress, profileAddress)) {
    // Recipient must have verified ethereum address
    walletError = "Incorrect wallet address"
    return
  }
  isLoading = true
  const signer = getSigner()
  subscriptionConfig = await getSubscriptionConfig(
    blockchain.contract_address,
    signer,
    profileAddress,
  )
  if (subscriptionConfig !== null) {
    subscriptionsEnabled = true
    // Ensure server is aware of subscription configuration
    await onSubscriptionsEnabled(
      ensureAuthToken(),
      blockchain.chain_id,
    )
    await loadSubscriptionOption()
    subscriptions = await getReceivedSubscriptions(
      ensureAuthToken(),
      profile.id,
      true,
    )
  } else {
    subscriptionsEnabled = false
    subscriptionToken = await getSubscriptionToken(
      blockchain.contract_address,
      signer,
    )
  }
  isLoading = false
}

function canVerifyEthereumAddress(): boolean {
  return walletAddress !== null && profile.getVerifiedEthereumAddress() === null
}

async function onVerifyEthereumAddress() {
  await verifyEthereumAddress()
  await loadSubscriptionSettings()
}

function canEnableSubscriptions(): boolean {
  return (
    profile.getVerifiedEthereumAddress() !== null &&
    subscriptionsEnabled === false &&
    subscriptionToken !== null
  )
}

async function onEnableSubscriptions() {
  if (
    walletAddress === null ||
    !blockchain?.contract_address ||
    subscriptionToken === null
  ) {
    return
  }
  isLoading = true
  const signer = getSigner()
  const authToken = ensureAuthToken()
  const pricePerSec = getPricePerSec(
    subscriptionPrice,
    subscriptionToken.decimals,
  )
  const signature = await getSubscriptionAuthorization(authToken, pricePerSec)
  let transaction
  try {
    transaction = await configureSubscriptions(
      blockchain.contract_address,
      signer,
      walletAddress,
      pricePerSec,
      signature,
    )
  } catch (error) {
    console.error(error)
    isLoading = false
    return
  }
  await transaction.wait()
  subscriptionsEnabled = true
  subscriptionConfig = await getSubscriptionConfig(
    blockchain.contract_address,
    signer,
    walletAddress,
  )
  const user = await onSubscriptionsEnabled(
    authToken,
    blockchain.chain_id,
  )
  setCurrentUser(user)
  // Reload subscription option info
  await loadSubscriptionOption()
  isLoading = false
}

function isSubscriptionActive(subscription: Subscription): boolean {
  return !isPastDate(subscription.expires_at)
}

function onSubscriberSelected(subscription: Subscription) {
  if (subscription.sender_address !== null) {
    subscriberAddress = subscription.sender_address
    subscriptionState = null
  }
}

async function onCheckSubsciptionState() {
  if (
    !walletAddress ||
    !blockchain?.contract_address ||
    !subscriberAddress
  ) {
    return
  }
  isLoading = true
  const signer = getSigner()
  subscriptionState = await getSubscriptionState(
    blockchain.contract_address,
    signer,
    subscriberAddress,
    walletAddress,
  )
  isLoading = false
}

async function onWithdrawReceived() {
  if (
    !blockchain?.contract_address ||
    !subscriberAddress
  ) {
    return
  }
  isLoading = true
  const signer = getSigner()
  await withdrawReceived(
    blockchain.contract_address,
    signer,
    subscriberAddress,
  )
  subscriptionState = null
  isLoading = false
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/theme";

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

  .info-item {
    font-size: 16px;
    font-weight: bold;
  }
}

.setup {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;

  .price {
    align-items: center;
    display: flex;
    font-size: 16px;
    gap: $input-padding;
    justify-content: center;

    label {
      font-weight: bold;
    }

    input {
      width: 100px;
    }
  }
}

.withdraw {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;

  h2 {
    font-size: 20px;
  }

  .subscriber,
  input {
    width: 400px;
  }

  .subscriber.expired {
    opacity: 0.5;
  }
}

.loader {
  margin: 0 auto;
}
</style>
