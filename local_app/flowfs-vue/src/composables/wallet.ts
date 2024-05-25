import { Signer } from "ethers"
import { ref } from "vue"

import { useInstanceInfo } from "@/composables/instance"
import {
  getWallet,
  getWeb3Provider,
  parseCAIP2_ChainId,
} from "@/utils/ethereum"

const walletAddress = ref<string | null>(null)
const walletError = ref<string | null>(null)

function disconnectWallet() {
  walletAddress.value = null
  walletError.value = null
}

async function connectWallet(): Promise<void> {
  const { getBlockchainInfo, getEthereumChainMetadata } = useInstanceInfo()
  const blockchain = getBlockchainInfo()
  if (!blockchain) {
    throw new Error("blockchain integration disabled")
  }
  let provider
  try {
    provider = getWeb3Provider()
  } catch (error) {
    walletError.value = "Wallet not found"
    return
  }

  const instanceChainId = parseCAIP2_ChainId(blockchain.chain_id)
  const chainMetadata = getEthereumChainMetadata(blockchain)
  try {
    // https://eips.ethereum.org/EIPS/eip-3326
    await provider.send(
      "wallet_switchEthereumChain",
      [{ chainId: instanceChainId }],
    )
  } catch (switchError: any) {
    // This error code indicates that the chain has not been added to MetaMask
    if (switchError.code === 4902 && chainMetadata) {
      // https://eips.ethereum.org/EIPS/eip-3085
      const ethereumChainParams = {
        chainId: instanceChainId,
        chainName: chainMetadata.chain_name,
        rpcUrls: [chainMetadata.public_api_url],
        blockExplorerUrls: chainMetadata.explorer_url ? [chainMetadata.explorer_url] : [],
        nativeCurrency: {
          name: chainMetadata.currency_name,
          symbol: chainMetadata.currency_symbol,
          decimals: chainMetadata.currency_decimals,
        },
      }
      try {
        await provider.send(
          "wallet_addEthereumChain",
          [ethereumChainParams],
        )
      } catch (addError) {
        walletError.value = "Incorrect network"
        return
      }
    } else {
      walletError.value = "Incorrect network"
      return
    }
  }

  const signer = await getWallet(provider)
  if (!signer) {
    walletError.value = "Wallet not connected"
    return
  }
  // Connected
  walletAddress.value = await signer.getAddress()
  walletError.value = null

  const walletProvider = provider.provider as any
  walletProvider.on("chainChanged", (chainId: string) => {
    if (chainId !== instanceChainId) {
      disconnectWallet()
    }
  })
  walletProvider.on("accountsChanged", () => {
    disconnectWallet()
  })
}

// Can't use reactive signer object because it doesn't work with ethers.js
function getSigner(): Signer {
  const provider = getWeb3Provider()
  return provider.getSigner()
}

export function useWallet() {
  return {
    connectWallet,
    walletAddress,
    walletError,
    getSigner,
  }
}
