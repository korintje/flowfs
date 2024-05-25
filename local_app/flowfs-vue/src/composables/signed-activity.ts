import canonicalize from "canonicalize"

import { sendSignedActivity } from "@/api/c2s"
import { getUnsignedUpdate } from "@/api/users"
import { useCurrentUser } from "@/composables/user"
import { createDidFromEthereumAddress } from "@/utils/did"
import { hexToBytes, encodeMultibase } from "@/utils/encodings"
import { getWallet, getWalletSignature } from "@/utils/ethereum"

function addIntegrityProof(
  object: any,
  verificationMethod: string,
  signature: string,
): any {
  const signatureBytes = hexToBytes(signature)
  const proof = {
    type: "MitraJcsEip191Signature2022",
    proofPurpose: "assertionMethod",
    verificationMethod: verificationMethod,
    created: (new Date()).toISOString(),
    proofValue: encodeMultibase(signatureBytes),
  }
  object.proof = proof
  return object
}

async function signUpdateActivity(): Promise<void> {
  const { ensureAuthToken, ensureCurrentUser } = useCurrentUser()
  if (!confirm("This action will sign a message with your wallet and send it to your followers. Continue?")) {
    return
  }
  const signer = await getWallet()
  if (!signer) {
    return
  }
  const walletAddress = await signer.getAddress()
  const authToken = ensureAuthToken()
  const { value } = await getUnsignedUpdate(authToken)
  const message = canonicalize(value)
  if (!message) {
    throw new Error("canonicalization error")
  }
  const signature = await getWalletSignature(signer, message)
  const signatureHex = signature.replace(/0x/, "")
  const verificationMethod = createDidFromEthereumAddress(walletAddress)
  const signedValue = addIntegrityProof(value, verificationMethod, signatureHex)
  const currentUser = ensureCurrentUser()
  await sendSignedActivity(
    currentUser.username,
    signedValue,
  )
}

export function useSignedActivity() {
  return {
    signUpdateActivity,
  }
}
