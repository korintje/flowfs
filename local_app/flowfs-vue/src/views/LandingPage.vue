<template>
  <div class="landing-page wide">
    <div class="instance-group">
      <div v-if="instance" class="instance-info">
        <h1 class="instance-title">{{ instance.title }}</h1>
        <div class="instance-description">
          {{ instance.short_description }}
        </div>
        <router-link
          :to="{ name: 'about' }"
          class="instance-link"
        >
          Learn more <icon-arrow-long class="arrow"></icon-arrow-long>
        </router-link>
        <router-link
          v-if="instance.allow_unauthenticated.timeline_local"
          class="instance-link"
          :to="{ name: 'local' }"
        >
          Explore <icon-arrow-long class="arrow"></icon-arrow-long>
        </router-link>
      </div>
      <div v-if="instance" class="login-form-group">
        <div
          class="login-type"
          v-if="allowedAuthenticationMethods.length > 1"
        >
          <button
            v-for="authType in allowedAuthenticationMethods"
            :key="authType"
            :class="{ active: loginType === authType }"
            @click.prevent="loginType = authType; loginErrorMessage = null"
          >
            <template v-if="authType === 'password'">Password</template>
            <template v-else-if="authType === 'eip4361'">Ethereum</template>
            <template v-else-if="authType === 'caip122_monero'">Monero</template>
          </button>
        </div>
        <form class="login-form">
          <div v-if="isLoading" class="login-form-loader">
            <loader></loader>
          </div>
          <div class="form-control" v-if="!isRegistered || loginType == 'password'">
            <div class="input-group">
              <input
                type="text"
                id="username"
                v-model="username"
                required
                placeholder="Username"
              >
              <div class="addon">@{{ instance.uri }}</div>
            </div>
            <div
              v-if="!isUsernameValid()"
              class="form-message"
              :class="{ error: !isUsernameValid() }"
            >
              Only lowercase letters, numbers and underscores are allowed.
            </div>
          </div>
          <div class="form-control" v-if="loginType === 'password'">
            <input
              id="password"
              type="password"
              v-model="password"
              required
              placeholder="Password"
            >
          </div>
          <div class="form-control" v-if="loginType === 'caip122_monero'">
            <input
              id="monero-address"
              type="text"
              v-model="moneroAddress"
              required
              placeholder="Monero address"
            >
            <div class="form-message">
              The primary address of your Monero account.
            </div>
          </div>
          <div v-if="loginType === 'caip122_monero' && moneroAddress">
            <textarea
              id="monero-message"
              type="text"
              :value="moneroCaip122Message || ''"
              @click="selectCaip122Message"
              readonly
            >
            </textarea>
            <div class="form-message">
              Sign this message using your Monero wallet.
            </div>
          </div>
          <div v-if="loginType === 'caip122_monero'">
            <input
              id="monero-signature"
              type="password"
              v-model="moneroSignature"
              required
              placeholder="Signature"
            >
          </div>
          <div class="form-control" v-if="!instance.registrations && !isRegistered">
            <input
              type="text"
              id="invite-token"
              v-model="inviteCode"
              required
              placeholder="Enter the invite code"
            >
          </div>
          <div class="wallet-required" v-if="loginType === 'eip4361'">
            <icon-ethereum></icon-ethereum>
            <router-link :to="{ name: 'ethereum' }">Ethereum Wallet</router-link> is required
          </div>
          <button
            v-if="isRegistered"
            type="submit"
            :disabled="!isLoginFormValid()"
            @click.prevent="login()"
          >
            Sign in
          </button>
          <button
            v-else
            type="submit"
            :disabled="!isLoginFormValid()"
            @click.prevent="register()"
          >
            Sign Up
          </button>
          <div class="error-message" v-if="loginErrorMessage" >{{ loginErrorMessage }}</div>
        </form>
        <div class="switch-mode">
          <template v-if="isRegistered">Don't have an account?</template>
          <template v-else>Already registered?</template>
          &thinsp;
          <button @click.prevent="isRegistered = !isRegistered; loginErrorMessage = null">
            <template v-if="isRegistered">Sign Up</template>
            <template v-else>Sign In</template>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from "vue"
import { $, $$, $ref } from "vue/macros"
import { useRouter } from "vue-router"

import {
  createUser,
  getAccessToken,
  getCurrentUser,
  AuthenticationMethod,
} from "@/api/users"
import IconArrowLong from "@/assets/arrow_long.svg?component"
import IconEthereum from "@/assets/forkawesome/ethereum.svg?component"
import Loader from "@/components/Loader.vue"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"
import {
  createEip4361_SignedMessage,
  getWallet,
  hasEthereumWallet,
} from "@/utils/ethereum"
import { createMoneroCaip122Message } from "@/utils/monero"

const router = useRouter()
const { setCurrentUser, setAuthToken } = useCurrentUser()
const { getBlockchainInfo, instance } = $(useInstanceInfo())

const isRegistered = $ref(true)
const username = $ref("")
const password = $ref<string | null>(null)
const moneroAddress = $ref<string | null>(null)
const moneroSignature = $ref<string | null>(null)
const inviteCode = $ref<string | null>(null)
let loginType = $ref<AuthenticationMethod>(AuthenticationMethod.Password)
let isLoading = $ref(false)
let loginErrorMessage = $ref<string | null>(null)

function isWalletRequired(): boolean {
  if (!instance) {
    return false
  }
  const blockchain = getBlockchainInfo()
  return Boolean(blockchain?.features.gate)
}

const allowedAuthenticationMethods = computed(() => {
  if (!instance) {
    return []
  }
  if (isWalletRequired()) {
    return [AuthenticationMethod.Eip4361]
  }
  return instance.authentication_methods
})

watch($$(instance), () => {
  if (
    allowedAuthenticationMethods.value.includes(AuthenticationMethod.Eip4361) &&
    (hasEthereumWallet() || isWalletRequired())
  ) {
    // Switch to EIP-4361 if wallet is present or
    // if registration is token-gated
    loginType = AuthenticationMethod.Eip4361
  }
}, { immediate: true })

const moneroCaip122Message = computed(() => {
  if (!instance || !moneroAddress) {
    return null
  }
  return createMoneroCaip122Message(
    moneroAddress,
    instance.uri,
    instance.login_message,
  )
})

function selectCaip122Message(event: MouseEvent) {
  const textarea = event?.target as HTMLTextAreaElement
  textarea.select()
}

function isUsernameValid(): boolean {
  if (!username) {
    return true
  }
  return /^[a-z0-9_]+$/.test(username)
}

function isLoginFormValid(): boolean {
  if (!instance) {
    return false
  }
  if (isRegistered) {
    if (loginType === AuthenticationMethod.Password) {
      return Boolean(username) && isUsernameValid() && Boolean(password)
    } else if (loginType === AuthenticationMethod.Caip122Monero) {
      return Boolean(moneroSignature)
    } else {
      return true
    }
  } else {
    const inviteCodeValid = instance.registrations ? true : Boolean(inviteCode)
    if (!username || !isUsernameValid()) {
      return false
    }
    if (loginType === AuthenticationMethod.Password) {
      return Boolean(password) && inviteCodeValid
    } else if (loginType === AuthenticationMethod.Caip122Monero) {
      return Boolean(moneroSignature) && inviteCodeValid
    } else {
      return inviteCodeValid
    }
  }
}

async function register() {
  loginErrorMessage = null
  if (!instance) {
    return
  }
  let userData
  let loginData
  if (loginType === AuthenticationMethod.Password) {
    userData = {
      username,
      password,
      message: null,
      signature: null,
      invite_code: inviteCode,
    }
    loginData = { username, password, message: null, signature: null }
  } else if (loginType === AuthenticationMethod.Eip4361) {
    const signer = await getWallet()
    if (!signer) {
      loginErrorMessage = "wallet not found"
      return
    }
    const { message, signature } = await createEip4361_SignedMessage(
      signer,
      instance.uri,
      instance.login_message,
    )
    userData = {
      username,
      password: null,
      message,
      signature,
      invite_code: inviteCode,
    }
    loginData = { username: null, password: null, message, signature }
  } else if (loginType === AuthenticationMethod.Caip122Monero) {
    const message = moneroCaip122Message.value
    const signature = moneroSignature
    userData = {
      username,
      password: null,
      message,
      signature,
      invite_code: inviteCode,
    }
    loginData = { username: null, password: null, message, signature }
  } else {
    throw new Error("invalid login type")
  }
  isLoading = true
  let user
  let authToken
  try {
    user = await createUser(loginType, userData)
    authToken = await getAccessToken(loginType, loginData)
  } catch (error: any) {
    isLoading = false
    loginErrorMessage = error.message
    return
  }
  setCurrentUser(user)
  setAuthToken(authToken)
  isLoading = false
  router.push({ name: "home" })
}

async function login() {
  loginErrorMessage = null
  if (!instance) {
    return
  }
  let loginData
  if (loginType === AuthenticationMethod.Password) {
    loginData = { username, password, message: null, signature: null }
  } else if (loginType === AuthenticationMethod.Eip4361) {
    const signer = await getWallet()
    if (!signer) {
      loginErrorMessage = "wallet not found"
      return
    }
    const { message, signature } = await createEip4361_SignedMessage(
      signer,
      instance.uri,
      instance.login_message,
    )
    loginData = { username: null, password: null, message, signature }
  } else if (loginType === AuthenticationMethod.Caip122Monero) {
    const message = moneroCaip122Message.value
    const signature = moneroSignature
    loginData = {
      username: null,
      password: null,
      message,
      signature,
    }
  } else {
    throw new Error("invalid login type")
  }
  isLoading = true
  let user
  let authToken
  try {
    authToken = await getAccessToken(loginType, loginData)
    user = await getCurrentUser(authToken)
  } catch (error: any) {
    isLoading = false
    loginErrorMessage = error.message
    return
  }
  setCurrentUser(user)
  setAuthToken(authToken)
  isLoading = false
  router.push({ name: "home" })
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/theme";

.landing-page {
  background-color: $landing-background-color;
  background-image: $landing-background-image;
  background-repeat: no-repeat;
  background-size: cover;
  box-sizing: border-box;
  color: $landing-text-color;
  min-height: 100vh;
  padding-top: 20vh;
}

.instance-group {
  align-items: flex-start;
  display: flex;
  flex-direction: row;
  gap: $content-gap;
  justify-content: space-between;
  margin: 0 auto;
  max-width: $wide-content-width + $content-gap + $wide-sidebar-width;
}

.instance-info {
  display: flex;
  flex-direction: column;
  font-size: 24px;
  gap: 20px;
  max-width: $wide-content-width;
  min-width: 0;
}

.instance-title {
  font-size: 90px;
  font-weight: bold;
  margin: 0;
  text-transform: uppercase;
  word-wrap: break-word;
}

.instance-link {
  color: $landing-text-color;

  .arrow {
    fill: $landing-accent-color;
    height: 1em;
    vertical-align: middle;
    width: auto;

    &:hover {
      fill: $landing-text-color;
    }
  }
}

.login-form-group {
  background-color: $landing-block-background-color;
  border-radius: 10px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 15px;
  max-width: 100%;
  min-width: $wide-sidebar-width - 50px;
  padding: 30px;
  width: $wide-sidebar-width;
}

.login-type {
  border-radius: 10px;
  display: flex;

  button {
    border: 1px solid $landing-btn-background-color;
    color: $landing-btn-text-color;
    padding: 10px;
    text-align: center;
    width: 100%;

    &:first-child {
      border-bottom-left-radius: 10px;
      border-top-left-radius: 10px;
    }

    &:last-child {
      border-bottom-right-radius: 10px;
      border-top-right-radius: 10px;
    }

    &.active {
      background-color: $landing-btn-background-color;
    }
  }
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
  position: relative;

  input,
  textarea,
  .addon {
    background-color: $landing-input-background-color;
    border: none;
    line-height: 18px;
    padding: 15px;
  }

  input,
  textarea {
    border-radius: 10px;
    color: $landing-text-color;
    min-width: 100px;

    &::placeholder {
      color: $landing-input-addon-color;
    }
  }

  .input-group {
    display: flex;
    flex-direction: row;

    input {
      border-radius: 10px 0 0 10px;
      min-width: 0;
    }

    .addon {
      border-radius: 0 10px 10px 0;
      color: $landing-input-addon-color;
      flex-shrink: 0;
      max-width: 40%;
      overflow: hidden;
      padding-left: 0;
      text-align: right;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  .form-message {
    font-size: 12px;
    margin-top: 3px;
    padding: 0 15px;

    &.error {
      color: $error-color;
    }
  }

  button[type="submit"] {
    background: $landing-login-btn-background-color;
    border: none;
    border-radius: 10px;
    box-shadow: 0 2px 16px -5px $landing-login-btn-shadow-color;
    color: $landing-text-color;
    cursor: pointer;
    display: block;
    font-size: 20px;
    font-weight: bold;
    height: 48px;
    padding: 10px 60px;
    text-align: center;

    &:not([disabled]):hover {
      background: $landing-login-btn-background-hover-color;
    }
  }

  .error-message {
    color: $error-color;
    margin-top: 10px;
    text-align: center;
  }

  .wallet-required {
    align-items: center;
    display: flex;
    flex-direction: row;
    gap: 0.4em;
    justify-content: center;

    svg {
      fill: var(--btn-text-color);
      height: 1em;
      width: 1em;
    }

    a {
      color: $landing-text-color;
      text-decoration: underline;
    }
  }
}

.login-form-loader {
  bottom: 0;
  display: flex;
  left: 0;
  position: absolute;
  right: 0;
  top: 0;
  z-index: 1;

  .loader {
    margin: auto;
  }
}

.switch-mode {
  margin-top: 15px;
  text-align: center;

  button {
    color: $landing-text-color;
    text-decoration: underline;
  }
}

@media screen and (max-width: $screen-breakpoint-medium) {
  .login-form-group {
    padding: 25px;
  }
}

@media screen and (max-width: $screen-breakpoint-small) {
  .landing-page {
    padding-top: $content-gap;
  }

  .instance-group {
    flex-wrap: wrap;
    justify-content: flex-start;
  }

  .login-form-group {
    margin-right: auto;
    min-width: auto;
  }
}
</style>
