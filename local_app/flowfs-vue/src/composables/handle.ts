import { RouteLocationRaw } from "vue-router"

import { Mention } from "@/api/posts"
import { Profile } from "@/api/users"
import { useInstanceInfo } from "@/composables/instance"

export function useActorHandle() {
  const { instance } = useInstanceInfo()

  function getActorAddress(profile: Profile | Mention): string {
    if (profile.acct.includes("@")) {
      // Remote account
      return profile.acct
    }
    if (profile.acct.includes("://")) {
      // No webfinger address
      return profile.acct
    }
    if (instance.value === null) {
      return `${profile.username}`
    }
    return `${profile.username}@${instance.value.uri}`
  }

  function getActorHandle(profile: Profile | Mention): string {
    const address = getActorAddress(profile)
    if (address.includes("://")) {
      // No webfinger address
      return address
    }
    return `@${address}`
  }

  function getActorLocation(name: string, profile: Profile | Mention): RouteLocationRaw {
    let params
    if (profile.acct.includes("://")) {
      // No webfinger address
      params = { profileId: profile.id }
    } else {
      params = { acct: profile.acct }
      name = name + "-by-acct"
    }
    return { name, params }
  }

  return {
    getActorAddress,
    getActorHandle,
    getActorLocation,
  }
}
