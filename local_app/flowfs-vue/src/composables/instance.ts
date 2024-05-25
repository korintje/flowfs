import { ref } from "vue"

import {
  BlockchainInfo,
  EthereumChainMetadata,
  InstanceInfo,
  MoneroChainMetadata,
  getInstanceInfo,
} from "@/api/instance"
import { isEthereumChain, isMoneroChain } from "@/utils/cryptocurrencies"

const instance = ref<InstanceInfo | null>(null)

export function useInstanceInfo() {

  async function loadInstanceInfo(): Promise<void> {
    instance.value = await getInstanceInfo()
  }

  function getBlockchainInfo(): BlockchainInfo | null {
    return instance.value?.blockchains[0] || null
  }

  function getEthereumChainMetadata(
    blockchain: BlockchainInfo,
  ): EthereumChainMetadata | null {
    if (!isEthereumChain(blockchain.chain_id)) {
      throw new Error("invalid chain type")
    }
    if (blockchain.chain_metadata) {
      return blockchain.chain_metadata as EthereumChainMetadata
    } else {
      return null
    }
  }

  function getMoneroChainMetadata(
    blockchain: BlockchainInfo,
  ): MoneroChainMetadata | null {
    if (!isMoneroChain(blockchain.chain_id)) {
      throw new Error("invalid chain type")
    }
    if (blockchain.chain_metadata) {
      return blockchain.chain_metadata as MoneroChainMetadata
    } else {
      return null
    }
  }

  return {
    instance,
    loadInstanceInfo,
    getBlockchainInfo,
    getEthereumChainMetadata,
    getMoneroChainMetadata,
  }
}
