import { computed } from "vue"

import { useClientConfig, ConfigKey } from "@/composables/client-config"

enum Theme {
  Light = "light",
  Dark = "dark",
}

function defaultTheme(): Theme {
  // Doesn't work in Firefox if privacy.resistFingerprinting is set to true
  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return Theme.Dark
  } else {
    return Theme.Light
  }
}

export function useTheme() {
  const { getClientConfigKey, setClientConfigKey } = useClientConfig()

  const currentTheme = computed<Theme>(() => {
    const theme = getClientConfigKey(ConfigKey.Theme) || defaultTheme()
    return theme as Theme
  })

  function applyTheme(theme: Theme) {
    document.documentElement.setAttribute("data-theme", theme)
  }

  function loadTheme() {
    applyTheme(currentTheme.value)
  }

  const darkModeEnabled = computed(() => currentTheme.value === Theme.Dark)

  async function toggleDarkMode() {
    let theme
    if (currentTheme.value === Theme.Light) {
      theme = Theme.Dark
    } else {
      theme = Theme.Light
    }
    applyTheme(theme)
    await setClientConfigKey(ConfigKey.Theme, theme)
  }

  return {
    darkModeEnabled,
    loadTheme,
    toggleDarkMode,
  }
}
