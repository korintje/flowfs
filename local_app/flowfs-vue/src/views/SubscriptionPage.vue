<template>
  <sidebar-layout v-if="profile">
    <template #content>
      <h1>Subscription</h1>
      <subscription-ethereum v-if="isEthereum()" :profile="profile"></subscription-ethereum>
      <subscription-monero v-else-if="isMonero()" :profile="profile"></subscription-monero>
      <div v-else>No subscription info</div>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { $, $ref } from "vue/macros"
import { useRoute } from "vue-router"

import {
  isRemoteProfile,
  lookupProfile,
  getProfile,
  Profile,
  ProfilePaymentOption,
} from "@/api/users"
import SidebarLayout from "@/components/SidebarLayout.vue"
import SubscriptionEthereum from "@/components/SubscriptionEthereum.vue"
import SubscriptionMonero from "@/components/SubscriptionMonero.vue"
import { useSubscribe } from "@/composables/subscribe"
import { useCurrentUser } from "@/composables/user"
import { isEthereumChain, isMoneroChain } from "@/utils/cryptocurrencies"

const route = useRoute()
const { authToken, currentUser } = $(useCurrentUser())
const { getSubscriptionOption } = useSubscribe()
let profile = $ref<Profile | null>(null)
let subscriptionOption = $ref<ProfilePaymentOption | null>(null)

onMounted(async () => {
  // Recipient
  if (route.params.acct) {
    profile = await lookupProfile(
      authToken,
      route.params.acct as string,
    )
  } else {
    profile = await getProfile(
      authToken,
      route.params.profileId as string,
    )
  }
  if (isRemoteProfile(profile) && currentUser === null) {
    // Only authenticated users may view remote subscriptions
    return
  }
  subscriptionOption = getSubscriptionOption(profile)
})

function isEthereum(): boolean {
  if (!subscriptionOption?.chain_id) {
    return false
  }
  return isEthereumChain(subscriptionOption.chain_id)
}

function isMonero(): boolean {
  if (!subscriptionOption?.chain_id) {
    return false
  }
  return isMoneroChain(subscriptionOption.chain_id)
}
</script>
