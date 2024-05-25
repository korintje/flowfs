<template>
  <static-page class="wide">
    <template #heading>Ethereum</template>
    <template #text>
      <p>
        You can register on this server with your <a href="https://ethereum.org/en/wallets/find-wallet/?filters=has_explore_dapps" target="_blank" rel="noopener">Ethereum wallet</a>.
        <a href="https://metamask.io/" target="_blank" rel="noopener">MetaMask</a> is a good choice for beginners.
        Your wallet address will be stored for authentication purposes and will not be exposed unless you later complete the address verification procedure.
        In that case, your address will be publicly associated with your account.
        To protect your privacy, it is recommended that you create a fresh address and don't link it to your other activities.
        This can be achieved by using an exchange <a href="http://kycnot.me/" target="_blank" rel="noopener">that doesn't require identity verification</a>.
        For general privacy recommendations, visit <a href="https://www.privacyguides.org/tools/" target="_blank" rel="noopener">PrivacyGuides</a> website.
      </p>
    </template>
  </static-page>
</template>

<script setup lang="ts">
import StaticPage from "@/components/StaticPage.vue"
</script>
