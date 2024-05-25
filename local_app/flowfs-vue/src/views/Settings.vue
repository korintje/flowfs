<template>
  <sidebar-layout v-if="currentUser">
    <template #content>
      <h1>Settings</h1>
      <section>
        <h2>Profile</h2>
        <router-link
          class="edit-profile btn"
          :to="{ name: 'settings-profile' }"
        >
          Edit profile
        </router-link>
      </section>
      <section>
        <h2>Appearance</h2>
        <div class="appearance-checkbox">
          <input
            type="checkbox"
            id="dark-mode"
            :checked="darkModeEnabled"
            @change="onToggleDarkMode()"
            :disabled="isLoading"
          >
          <label for="dark-mode">Enable dark mode</label>
        </div>
        <div class="appearance-checkbox">
          <input
            type="checkbox"
            id="content-warnings"
            :checked="contentWarningsEnabled"
            @change="onToggleContentWarnings()"
            :disabled="isLoading"
          >
          <label for="content-warnings">Enable content warnings</label>
        </div>
        <div class="appearance-checkbox">
          <input
            type="checkbox"
            id="ctrl-enter"
            :checked="ctrlEnterEnabled"
            @change="onToggleCtrlEnter()"
            :disabled="isLoading"
          >
          <label for="ctrl-enter">Send messages with Ctrl+Enter</label>
        </div>
      </section>
      <section>
        <h2>Authentication</h2>
        <div class="authentication-methods">
          Enabled authentication methods:
          <span v-for="(method, index) in currentUser.authentication_methods" :key="method">
            <template v-if="method === 'password'">password</template>
            <template v-else-if="method === 'eip4361'">EIP-4361</template>
            <template v-else-if="method === 'caip122_monero'">CAIP-122 (Monero)</template>
            <template v-if="index !== currentUser.authentication_methods.length - 1">, </template>
          </span>
        </div>
        <h3>Change password</h3>
        <form @submit.prevent="onChangePassword()">
          <div class="input-group">
            <label for="new-password">New password</label>
            <input id="new-password" type="password" v-model="newPassword">
          </div>
          <div class="input-group">
            <label for="new-password-confirmation">New password (confirmation)</label>
            <input id="new-password-confirmation" type="password" v-model="newPasswordConfirmation">
          </div>
          <button
            type="submit"
            class="btn"
            :disabled="!canChangePassword()"
          >
            Save
          </button>
          <div class="password-form-message" v-if="passwordFormMessage">
            {{ passwordFormMessage }}
          </div>
        </form>
      </section>
      <section>
        <h2>Identities</h2>
        <router-link class="btn" :to="{ name: 'settings-aliases' }">
          Manage identities
        </router-link>
      </section>
      <section>
        <h2>Export</h2>
        <table class="export">
          <tr>
            <td>Follows</td>
            <td>{{ currentUser.following_count }}</td>
            <td>
              <a @click="onExportFollows()">download</a>
            </td>
          </tr>
          <tr>
            <td>Followers</td>
            <td>{{ currentUser.followers_count }}</td>
            <td>
              <a @click="onExportFollowers()">download</a>
            </td>
          </tr>
        </table>
      </section>
      <section>
        <h2>Delete account</h2>
        <button
          class="btn"
          @click="onDeleteAccount()"
        >
          Delete account
        </button>
      </section>
      <section>
        <h2>Experiments</h2>
        <details class="experiments">
          <summary>This section contains experimental features. Use at your own risk.</summary>
          <div class="experiments-wrapper">
            <router-link class="btn" :to="{ name: 'import-follows' }">
              Import follows
            </router-link>
            <router-link class="btn" :to="{ name: 'import-followers' }">
              Import followers
            </router-link>
          </div>
        </details>
      </section>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { $ref } from "vue/macros"
import { useRouter } from "vue-router"

import {
  changePassword,
  deleteAccount,
  exportFollowers,
  exportFollows,
} from "@/api/settings"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useClientConfig, ConfigKey } from "@/composables/client-config"
import { useTheme } from "@/composables/theme"
import { useCurrentUser } from "@/composables/user"

const router = useRouter()
const {
  contentWarningsEnabled,
  ctrlEnterEnabled,
  setClientConfigKey,
} = useClientConfig()
const {
  currentUser,
  endSession,
  ensureAuthToken,
  setCurrentUser,
} = useCurrentUser()
const { darkModeEnabled, toggleDarkMode } = useTheme()
let newPassword = $ref("")
let newPasswordConfirmation = $ref("")
let passwordFormMessage = $ref<string | null>(null)
let isLoading = $ref(false)

async function onToggleDarkMode() {
  isLoading = true
  await toggleDarkMode()
  isLoading = false
}

async function onToggleContentWarnings() {
  isLoading = true
  await setClientConfigKey(
    ConfigKey.ContentWarningsEnabled,
    !contentWarningsEnabled.value,
  )
  isLoading = false
}

async function onToggleCtrlEnter() {
  isLoading = true
  await setClientConfigKey(
    ConfigKey.CtrlEnterEnabled,
    !ctrlEnterEnabled.value,
  )
  isLoading = false
}

function canChangePassword(): boolean {
  return newPassword && newPassword === newPasswordConfirmation
}

async function onChangePassword() {
  const authToken = ensureAuthToken()
  const user = await changePassword(authToken, newPassword)
  setCurrentUser(user)
  newPassword = ""
  newPasswordConfirmation = ""
  passwordFormMessage = "Password changed"
}

async function onDeleteAccount() {
  const authToken = ensureAuthToken()
  if (confirm("Are you sure you want to delete your account? This can not be undone.")) {
    await deleteAccount(authToken)
    endSession()
    router.push({ name: "landing-page" })
  }
}

async function onExportFollows() {
  const authToken = ensureAuthToken()
  await exportFollows(authToken)
}

async function onExportFollowers() {
  const authToken = ensureAuthToken()
  await exportFollowers(authToken)
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

section {
  margin-bottom: 3 * $block-outer-padding;
}

form {
  @include content-form;
}

.appearance-checkbox {
  margin-top: $block-outer-padding;
}

.authentication-methods {
  margin-bottom: $block-inner-padding;
}

.password-form-message {
  margin-top: $block-outer-padding;
}

.export {
  td {
    font-weight: bold;
    padding: 0 $block-inner-padding $block-inner-padding 0;

    &:last-child {
      font-weight: normal;
    }
  }

  a {
    color: var(--block-link-color);
  }
}

.experiments {
  summary {
    margin-bottom: $block-outer-padding;
  }

  .experiments-wrapper {
    align-items: flex-start;
    display: flex;
    flex-direction: column;
    gap: $block-outer-padding;
  }
}
</style>
