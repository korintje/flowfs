import { RouteLocationRaw } from "vue-router"

import { Profile, ProfilePaymentOption } from "@/api/users"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"

interface SubscriptionLink {
  type: "ethereum" | "monero",
  location: string | RouteLocationRaw,
}

export function useSubscribe() {
  const { getActorLocation } = useActorHandle()
  const { getBlockchainInfo } = useInstanceInfo()

  function getSubscriptionLink(profile: Profile): SubscriptionLink | null {
    for (const option of profile.payment_options) {
      if (
        option.type === "link" &&
        (option.name === "EthereumSubscription" || option.name === "MoneroSubscription") &&
        option.href
      ) {
        return {
          type: option.name === "EthereumSubscription" ? "ethereum" : "monero",
          location: option.href,
        }
      } else if (
        option.type === "ethereum-subscription" ||
        option.type === "monero-subscription"
      ) {
        const blockchain = getBlockchainInfo()
        if (!option.object_id && !blockchain?.features.subscriptions) {
          // Local subscription option, but subscription feature is disabled
          continue
        }
        return {
          type: option.type === "ethereum-subscription" ? "ethereum" : "monero",
          location: getActorLocation("profile-subscription", profile),
        }
      }
    }
    return null
  }

  function getSubscriptionOption(
    profile: Profile,
  ): ProfilePaymentOption | null {
    // Use first option if there are many
    const subscriptionOption = profile.payment_options.find((option) => {
      return option.type === "ethereum-subscription" || option.type === "monero-subscription"
    }) || null
    return subscriptionOption
  }

  return {
    getSubscriptionLink,
    getSubscriptionOption,
  }
}
