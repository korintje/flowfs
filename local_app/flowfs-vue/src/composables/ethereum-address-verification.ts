import { createIdentityProof, getIdentityClaim, User } from "@/api/users"
import { useCurrentUser } from "@/composables/user"
import { getWallet, getWalletSignature } from "@/utils/ethereum"

async function verifyEthereumAddress(): Promise<User | null> {
  const { ensureAuthToken, setCurrentUser } = useCurrentUser()
  if (!confirm("This action will link your wallet address to your profile. Continue?")) {
    return null
  }
  const signer = await getWallet()
  if (!signer) {
    return null
  }
  const walletAddress = await signer.getAddress()
  const authToken = ensureAuthToken()
  const proofType = "ethereum"
  const { did, claim, created_at } = await getIdentityClaim(
    authToken,
    proofType,
    walletAddress,
  )
  const signature = await getWalletSignature(signer, claim)
  const user = await createIdentityProof(
    authToken,
    proofType,
    did,
    signature,
    created_at,
  )
  setCurrentUser(user)
  return user
}

export function useEthereumAddressVerification() {
  return { verifyEthereumAddress }
}
