import { computed } from "vue"

import { updateClientConfig } from "@/api/settings"
import { ClientConfigValue } from "@/api/users"
import { useCurrentUser } from "@/composables/user"
import { APP_NAME } from "@/constants"

export enum ConfigKey {
  Theme = "theme",
  ContentWarningsEnabled = "contentWarningsEnabled",
  CtrlEnterEnabled = "ctrlEnterEnabled",
}

export function useClientConfig() {

  function getClientConfigKey(
    key: ConfigKey,
  ): ClientConfigValue | undefined {
    const { currentUser } = useCurrentUser()
    const clientConfig = currentUser.value?.client_config[APP_NAME] || {}
    const value = clientConfig[key]
    return value
  }

  async function setClientConfigKey(
    key: ConfigKey,
    value: ClientConfigValue,
  ) {
    const {
      ensureAuthToken,
      ensureCurrentUser,
      setCurrentUser,
    } = useCurrentUser()
    const currentUser = ensureCurrentUser()
    const clientConfig = currentUser.client_config[APP_NAME] || {}
    clientConfig[key] = value
    const authToken = ensureAuthToken()
    const user = await updateClientConfig(authToken, clientConfig)
    setCurrentUser(user)
  }

  const contentWarningsEnabled = computed(() => {
    const value = getClientConfigKey(ConfigKey.ContentWarningsEnabled) ?? true
    return value as boolean
  })

  const ctrlEnterEnabled = computed(() => {
    const value = getClientConfigKey(ConfigKey.CtrlEnterEnabled) ?? false
    return value as boolean
  })

  return {
    getClientConfigKey,
    setClientConfigKey,
    contentWarningsEnabled,
    ctrlEnterEnabled,
  }
}
