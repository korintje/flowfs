<template>
  <sidebar-layout>
    <template #content>
      <h1>Subscriber details</h1>
      <router-link
        v-if="subscriber"
        :to="getActorLocation('profile', subscriber)"
      >
        <profile-list-item :profile="subscriber">
          <template #profile-footer>
            <div v-if="isSubscriptionLoaded" class="subscription-info">
              <template v-if="subscription">
                <template v-if="isPastDate(subscription.expires_at)">Subscription expired</template>
                <template v-else>Subscription expires</template>
                {{ formatDate(subscription.expires_at) }}
              </template>
              <template v-else>
                Not subscribed yet
              </template>
            </div>
          </template>
        </profile-list-item>
      </router-link>
      <h2 v-if="subscriber">
        Grant subscription access
      </h2>
      <form
        v-if="subscriber"
        @submit.prevent="onExtendSubscription()"
      >
        <div class="input-group">
          <input
            type="number"
            placeholder="Duration"
            min="1"
            v-model="subscriptionDuration"
          >
          <select v-model="subscriptionDurationUnit">
            <option>months</option>
            <option>days</option>
          </select>
        </div>
        <button
          type="submit"
          class="btn"
          :disabled="!canExtendSubscription()"
        >
          Grant
        </button>
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
      </form>
      <loader v-if="isLoading"></loader>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { useRoute } from "vue-router"

import {
  findSubscription,
  SubscriptionDetails,
  SECONDS_IN_DAY,
  SECONDS_IN_MONTH,
} from "@/api/subscriptions-common"
import { extendSubscription } from "@/api/subscriptions-monero"
import { getProfile, Profile } from "@/api/users"
import Loader from "@/components/Loader.vue"
import ProfileListItem from "@/components/ProfileListItem.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useActorHandle } from "@/composables/handle"
import { useCurrentUser } from "@/composables/user"
import { formatDate, isPastDate } from "@/utils/dates"

const route = useRoute()
const { getActorLocation } = useActorHandle()
const { ensureAuthToken, ensureCurrentUser } = useCurrentUser()

const subscriber = ref<Profile | null>(null)
const subscription = ref<SubscriptionDetails | null>(null)
const subscriptionDuration = ref<number | "">(1)
const subscriptionDurationUnit = ref<"months" | "days">("months")
const errorMessage = ref<string | null>(null)
const isLoading = ref(false)
const isSubscriptionLoaded = ref(false)

async function onExtendSubscription() {
  if (!subscriber.value) {
    return
  }
  const units = subscriptionDuration.value === "" ? 0 : subscriptionDuration.value
  let duration
  switch (subscriptionDurationUnit.value) {
    case "months":
      duration = Math.round(units * SECONDS_IN_MONTH)
      break
    case "days":
      duration = Math.round(units * SECONDS_IN_DAY)
      break
  }
  isLoading.value = true
  try {
    subscription.value = await extendSubscription(
      ensureAuthToken(),
      subscriber.value.id,
      duration,
    )
  } catch (error: any) {
    errorMessage.value = error.message
    isLoading.value = false
    return
  }
  // Reset form
  subscriptionDuration.value = ""
  errorMessage.value = null
  isLoading.value = false
}

function canExtendSubscription(): boolean {
  return Boolean(subscriptionDuration.value)
}

onMounted(async () => {
  isLoading.value = true
  subscriber.value = await getProfile(
    ensureAuthToken(),
    route.params.profileId as string,
  )
  try {
    subscription.value = await findSubscription(
      subscriber.value.id,
      ensureCurrentUser().id,
    )
  } catch (error: any) {
    if (error.message !== "subscription not found") {
      isLoading.value = false
      throw error
    }
  }
  isLoading.value = false
  isSubscriptionLoaded.value = true
})
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.subscription-info {
  margin-top: $block-inner-padding;
}

h2 {
  margin-top: $block-outer-padding;
}

form {
  @include content-form;

  .input-group {
    align-items: center;
    display: flex;
    flex-direction: row;
    gap: $input-padding;

    select {
      background-color: var(--block-background-color);
      border: 1px solid var(--block-background-color);
      border-radius: $btn-border-radius;
      color: var(--text-color);
      padding: $input-padding;
    }
  }
}

.loader {
  margin: 0 auto;
}
</style>
