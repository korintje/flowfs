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
      </router-link>
    </div>
    <form class="sender" v-if="sender.id === ''">
      <input
        type="text"
        v-model="senderAcct"
        placeholder="Enter your username or Fediverse address (username@example.org)"
      >
      <button
        type="submit"
        class="btn"
        :disabled="!senderAcct"
        @click.prevent="identifySender()"
      >
        Find profile
      </button>
      <span class="sender-error">{{ senderError }}</span>
    </form>
    <div class="info" v-if="subscriptionOption !== null && sender.id !== ''">
      <template v-if="subscriptionPrice">
        <div class="price">
          {{ subscriptionPrice }} XMR
          <span class="price-subtext">per month</span>
        </div>
        <div class="status">
          <template v-if="!isSubscribed()">
            You are not subscribed yet
          </template>
          <template v-else-if="subscriptionDetails">
            <div>
              Subscription expires
              {{ formatDate(subscriptionDetails.expires_at) }}
            </div>
          </template>
          <template v-else>
            Subscription is active
          </template>
        </div>
      </template>
      <template v-else>
        Subscription is not available.
      </template>
    </div>
    <form class="payment" v-if="canSubscribe()">
      <div class="duration" @click="editDuration()">
        <label for="duration">Duration</label>
        <input
          v-if="!isAmountEditable"
          type="number"
          id="duration"
          v-model="paymentDurationInputValue"
          min="1"
        >
        <span
          v-else
          class="editable-value"
          title="Click to edit"
        >
          {{ paymentDuration }}
        </span>
        <span>months</span>
      </div>
      <div class="payment-amount" @click="editAmount()">
        <label for="amount">Amount</label>
        <input
          v-if="isAmountEditable && subscriptionPrice"
          type="number"
          id="amount"
          v-model="paymentAmountInputValue"
          :step="subscriptionPrice"
          min="0"
        >
        <span
          v-else
          class="editable-value"
          title="Click to edit"
        >
          {{ formatXmrAmount(paymentAmount) }}
        </span>
        <span>XMR</span>
      </div>
      <div
        v-if="paymentMessage"
        class="payment-message"
        v-html="paymentMessage"
      >
      </div>
      <button
        type="submit"
        class="btn primary"
        :disabled="!canCreateInvoice()"
        @click.prevent="onCreateInvoice()"
      >
        <template v-if="!isSubscribed()">
          Pay
        </template>
        <template v-else>Extend</template>
      </button>
    </form>
    <div class="invoice" v-if="invoice">
      <template v-if="invoice.status === 'open' || invoice.status === 'underpaid'">
        <div class="payment-header">
          Please send {{ formatXmrAmount(invoice.amount) }} XMR to this address:
          <a
            class="payment-request-toggle"
            title="Show additional information"
            @click="paymentRequestVisible = !paymentRequestVisible"
          >
            <icon-chevron-down></icon-chevron-down>
          </a>
        </div>
        <a
          class="payment-address"
          :href="getPaymentUri(invoice)"
        >
          {{ invoice.payment_address }}
        </a>
        <code
          v-if="paymentRequestVisible"
          class="payment-request"
        >
          {{ getPaymentRequest(invoice) }}
        </code>
        <qr-code :url="getPaymentUri(invoice)"></qr-code>
      </template>
      <div class="invoice-status">
        <template v-if="invoice.status === 'requested'">Awaiting response</template>
        <template v-else-if="invoice.status === 'open'">
          Waiting for payment ({{ getPaymentMinutesLeft(invoice) }} minutes left)
        </template>
        <template v-else-if="invoice.status === 'paid' || invoice.status === 'forwarded' || invoice.status === 'failed'">Processing payment</template>
        <template v-else-if="invoice.status === 'timeout'">Payment timed out</template>
        <template v-else-if="invoice.status === 'cancelled'">Payment cancelled</template>
        <template v-else-if="invoice.status === 'underpaid'">Payment amount is too small</template>
        <template v-else-if="invoice.status === 'completed'">Payment completed</template>
      </div>
      <button
        v-if="invoice.status === 'open'"
        class="btn"
        @click="onCancelInvoice()"
      >
        Cancel
      </button>
      <button
        v-else-if="invoice.status === 'completed' || invoice.status === 'timeout' || invoice.status === 'cancelled'"
        class="btn"
        @click="closeInvoice()"
      >
        OK
      </button>
    </div>
    <loader v-if="isLoaderVisible()"></loader>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { $, $computed, $ref } from "vue/macros"
import { useRoute } from "vue-router"
import { DateTime } from "luxon"

import { getRelationship, Relationship } from "@/api/relationships"
import { searchProfilesByAcct } from "@/api/search"
import {
  findSubscription,
  SubscriptionDetails,
  DAYS_IN_MONTH,
} from "@/api/subscriptions-common"
import {
  cancelInvoice,
  createInvoice,
  formatXmrAmount,
  getInvoice,
  getPaymentAmount,
  getPricePerMonth,
  getSubscriptionDuration,
  parseXmrAmount,
  Invoice,
} from "@/api/subscriptions-monero"
import { defaultProfile, Profile, ProfilePaymentOption, ProfileWrapper } from "@/api/users"
import IconArrowRight from "@/assets/feather/arrow-right.svg?component"
import IconChevronDown from "@/assets/feather/chevron-down.svg?component"
import Avatar from "@/components/Avatar.vue"
import Loader from "@/components/Loader.vue"
import QrCode from "@/components/QrCode.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useSubscribe } from "@/composables/subscribe"
import { useCurrentUser } from "@/composables/user"
import { formatDate, isPastDate } from "@/utils/dates"
import {
  createMoneroPaymentRequest,
  createMoneroPaymentUri,
} from "@/utils/monero"
import { isMoneroChain } from "@/utils/cryptocurrencies"

const INVOICE_ID_STORAGE_KEY = "invoice"
const PAYMENT_AMOUNT_MIN = 0.001

const props = defineProps<{
  profile: Profile,
}>()

const route = useRoute()
const { getActorLocation } = useActorHandle()
const { currentUser, ensureAuthToken } = $(useCurrentUser())
const { getBlockchainInfo, getMoneroChainMetadata } = useInstanceInfo()
const { getSubscriptionOption } = useSubscribe()
const recipient = new ProfileWrapper(props.profile)
const senderAcct = $ref("")
let senderError = $ref<string | null>(null)
let sender = $ref(new ProfileWrapper(currentUser || defaultProfile({ display_name: "You" })))
let subscriptionOption = $ref<ProfilePaymentOption | null>(null)
let subscriptionDetails = $ref<SubscriptionDetails | null>(null)
let relationship = $ref<Relationship | null>(null)
let paymentDurationInputValue = $ref<number | "">(1)
let paymentAmountInputValue = $ref<number | "">(0)
let isAmountEditable = $ref(false)
let invoice = $ref<Invoice | null>(null)
const paymentRequestVisible = ref(false)

let isLoading = $ref(false)

function getInvoiceIdStorageKey(): string {
  return `${INVOICE_ID_STORAGE_KEY}_${recipient.id}`
}

onMounted(async () => {
  isLoading = true
  const option = getSubscriptionOption(recipient)
  if (
    option !== null &&
    option.chain_id !== undefined &&
    isMoneroChain(option.chain_id)
  ) {
    subscriptionOption = option
    if (sender.id !== "") {
      await loadSubscriptionDetails()
      if (subscriptionDetails === null && currentUser !== null) {
        // Pre FEP-0837
        // Only authenticated users may view remote subscriptions
        relationship = await getRelationship(ensureAuthToken(), recipient.id)
      }
    }
  }
  isLoading = false
})

function isLoaderVisible(): boolean {
  return (
    isLoading ||
    invoice?.status === "paid" ||
    invoice?.status === "forwarded" ||
    invoice?.status === "failed"
  )
}

async function loadSubscriptionDetails() {
  subscriptionDetails = await findSubscription(sender.id, recipient.id)
  const invoiceId = (
    route.query.invoice_id ||
    localStorage.getItem(getInvoiceIdStorageKey())
  )
  if (invoiceId) {
    const lastInvoice = await getInvoice(invoiceId as string)
    if (
      lastInvoice.sender_id !== sender.id ||
      lastInvoice.recipient_id !== recipient.id
    ) {
      // Invoice created by different user
      return
    }
    invoice = lastInvoice
    if (
      invoice &&
      invoice.status !== "completed" &&
      invoice.status !== "cancelled"
    ) {
      watchInvoice()
    }
  }
}

// Human-readable subscription price
const subscriptionPrice = $computed<number | null>(() => {
  if (!subscriptionOption?.price) {
    return null
  }
  return getPricePerMonth(subscriptionOption.price)
})

async function identifySender() {
  if (!senderAcct) {
    return
  }
  isLoading = true
  let profiles
  try {
    profiles = await searchProfilesByAcct(
      null,
      senderAcct,
      true,
    )
  } catch (error: any) {
    if (error.message === "Too Many Requests") {
      senderError = "Too many requests"
      isLoading = false
      return
    }
    throw error
  }
  if (profiles.length > 1) {
    senderError = "Please provide full address"
  } else {
    const profile = profiles[0]
    if (profile && profile.id !== recipient.id) {
      sender = new ProfileWrapper(profile)
      senderError = null
      await loadSubscriptionDetails()
    } else {
      senderError = "Profile not found"
    }
  }
  isLoading = false
}

function isSubscribed(): boolean {
  if (subscriptionDetails === null) {
    if (!recipient.isLocal()) {
      // Pre-FEP-0837 remote subscriptions are simply relationships
      if (relationship === null) {
        return false
      }
      return relationship.subscription_to
    } else {
      return false
    }
  }
  return !isPastDate(subscriptionDetails.expires_at)
}

function canSubscribe(): boolean {
  return (
    sender.id !== "" &&
    sender.id !== recipient.id &&
    subscriptionOption !== null &&
    subscriptionPrice !== null &&
    invoice === null
  )
}

function editDuration() {
  if (!isAmountEditable) {
    return
  }
  paymentDurationInputValue = paymentDuration.value
  isAmountEditable = false
}

const paymentDuration = computed<number>(() => {
  if (!subscriptionOption?.price) {
    return 0
  }
  if (!isAmountEditable) {
    if (paymentDurationInputValue === "") {
      return 0
    }
    return paymentDurationInputValue
  }
  if (paymentAmountInputValue === "") {
    return 0
  }
  return getSubscriptionDuration(
    subscriptionOption.price,
    parseXmrAmount(paymentAmountInputValue),
  )
})

function editAmount() {
  if (isAmountEditable) {
    return
  }
  paymentAmountInputValue = formatXmrAmount(paymentAmount)
  isAmountEditable = true
}

const paymentAmount = $computed<number>(() => {
  if (!subscriptionOption?.price) {
    return 0
  }
  if (isAmountEditable) {
    if (paymentAmountInputValue === "") {
      return 0
    }
    return parseXmrAmount(paymentAmountInputValue)
  }
  if (paymentDurationInputValue === "") {
    return 0
  }
  return getPaymentAmount(
    subscriptionOption.price,
    paymentDurationInputValue,
  )
})

const paymentMessage = computed<string | null>(() => {
  if (!recipient.isLocal()) {
    return null
  }
  const blockchain = getBlockchainInfo()
  if (blockchain && blockchain.chain_id === subscriptionOption?.chain_id) {
    return getMoneroChainMetadata(blockchain)?.description || null
  } else {
    return null
  }
})

function canCreateInvoice(): boolean {
  return paymentAmount !== 0 && paymentAmount >= parseXmrAmount(PAYMENT_AMOUNT_MIN)
}

async function onCreateInvoice() {
  if (paymentAmount === 0) {
    return
  }
  if (!subscriptionOption || !subscriptionOption.chain_id) {
    return
  }
  isLoading = true
  try {
    invoice = await createInvoice(
      sender.id,
      recipient.id,
      subscriptionOption.chain_id,
      paymentAmount,
    )
  } catch (error: any) {
    alert(error.message)
    isLoading = false
    return
  }
  // Add invoice ID to current URL
  window.history.pushState(
    {},
    "",
    `${window.location.pathname}?invoice_id=${invoice.id}`,
  )
  localStorage.setItem(getInvoiceIdStorageKey(), invoice.id)
  isLoading = false
  watchInvoice()
}

function watchInvoice() {
  const watcher = setInterval(async () => {
    if (!invoice) {
      // Stop watching if invoice was closed
      clearInterval(watcher)
      return
    }
    invoice = await getInvoice(invoice.id)
    if (invoice.status === "completed") {
      // Stop watching and refresh subscription details
      clearInterval(watcher)
      subscriptionDetails = await findSubscription(sender.id, recipient.id)
    }
  }, 10000)
}

async function onCancelInvoice() {
  if (!invoice) {
    throw new Error("invoice doesn't exist")
  }
  await cancelInvoice(invoice.id)
  closeInvoice()
}

function closeInvoice() {
  invoice = null
  // Remove invoice ID from current URL
  window.history.pushState(
    {},
    "",
    window.location.pathname,
  )
  localStorage.removeItem(getInvoiceIdStorageKey())
}

function getPaymentUri(invoice: Invoice): string {
  return createMoneroPaymentUri(
    invoice.payment_address,
    formatXmrAmount(invoice.amount),
  )
}

function getPaymentRequest(invoice: Invoice): string {
  if (subscriptionPrice === null) {
    return ""
  }
  return createMoneroPaymentRequest(
    `Subscription to @${recipient.acct}`,
    subscriptionPrice,
    DAYS_IN_MONTH,
    invoice.payment_address,
    invoice.created_at,
  )
}

function getPaymentMinutesLeft(invoice: Invoice): number {
  const expiresAt = DateTime.fromISO(invoice.expires_at)
  const now = DateTime.now()
  const diff = expiresAt.diff(now)
  return Math.round(diff.as("minutes"))
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

.sender {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;
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
  .payment-amount {
    align-items: center;
    display: flex;
    font-size: 16px;
    font-weight: bold;
    gap: $input-padding;
    justify-content: center;

    input {
      font-size: inherit;
      width: 100px;
    }

    .editable-value {
      cursor: pointer;
    }
  }

  .payment-message {
    :deep(a) {
      @include block-link;
    }
  }
}

.invoice {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;
  padding-bottom: $block-inner-padding;

  .payment-header {
    box-sizing: border-box;
    padding: 0 $block-inner-padding;
    position: relative;
    width: 100%;
  }

  .payment-request-toggle {
    bottom: 0;
    position: absolute;
    right: 0;
    top: 0;
    width: 1em;

    svg {
      stroke: var(--text-color);
      vertical-align: middle;
    }
  }

  .payment-address {
    font-family: monospace;
    max-width: 100%;
    word-wrap: break-word;
  }

  .payment-request {
    font-family: monospace;
    max-width: 100%;
    word-wrap: break-word;
  }
}

.qr-wrapper {
  margin: 0 auto;
  max-width: 300px;
}

.loader {
  margin: 0 auto;
}
</style>
