import { BACKEND_URL } from "@/constants"
import { handleResponse, http, PAGE_SIZE } from "./common"
import { CustomEmoji } from "./emojis"

export const EXTRA_FIELD_COUNT_MAX = 10

export interface ProfileField {
  name: string;
  value: string;
  verified_at: string | null;
  is_legacy_proof: boolean,
}

export interface ProfilePaymentOption {
  type: string,
  name?: string,
  href?: string,
  chain_id?: string,
  price?: number,
  object_id?: string,
}

interface Source {
  note: string | null;
  fields: ProfileField[];
}

interface Role {
  id: number,
  name: string,
  permissions: string[],
}

export enum Permissions {
  CreatePost = "create_post",
  DeleteAnyProfile = "delete_any_profile",
  ManageSubscriptionOptions = "manage_subscription_options",
}

export enum AuthenticationMethod {
  Password = "password",
  Eip4361 = "eip4361",
  Caip122Monero = "caip122_monero",
}

export interface Profile {
  id: string;
  username: string;
  acct: string;
  actor_id: string,
  url: string;
  display_name: string | null;
  note: string | null;
  avatar: string | null;
  header: string | null;
  locked: boolean;
  mention_policy: "none" | "only_known" | "only_contacts",
  bot: boolean,
  identity_proofs: ProfileField[];
  payment_options: ProfilePaymentOption[];
  fields: ProfileField[];
  emojis: CustomEmoji[],

  followers_count: number;
  following_count: number;
  subscribers_count: number;
  statuses_count: number;
}

export function defaultProfile(fields: Partial<Profile> = {}): Profile {
  return {
    id: "",
    username: "",
    acct: "",
    actor_id: "",
    url: "",
    display_name: "",
    note: null,
    avatar: null,
    header: null,
    locked: false,
    mention_policy: "none",
    bot: false,
    identity_proofs: [],
    payment_options: [],
    fields: [],
    emojis: [],
    followers_count: 0,
    following_count: 0,
    subscribers_count: 0,
    statuses_count: 0,
    ...fields,
  }
}

export type ClientConfigValue = string | boolean

export interface User extends Profile {
  source: Source;
  role: Role,
  authentication_methods: AuthenticationMethod[];
  client_config: { [clientName: string]: { [property: string]: ClientConfigValue } },
}

export function hasAdminPermissions(user: User): boolean {
  return user.role.permissions.includes(Permissions.DeleteAnyProfile)
}

export function isRemoteProfile(profile: Profile): boolean {
  return profile.username !== profile.acct
}

export interface ProfileWrapper extends Profile {}
export class ProfileWrapper {

  constructor(source: Profile) {
    Object.assign(this, source)
  }

  getDisplayName(): string {
    let cleanDisplayName
    if (this.display_name) {
      // Replace control characters
      cleanDisplayName = this.display_name.replace(/\p{C}/gu, "")
    } else {
      cleanDisplayName = this.display_name
    }
    return cleanDisplayName || this.username
  }

  getVerifiedEthereumAddress(): string | null {
    for (const field of this.identity_proofs) {
      if (field.name === "$ETH") {
        return field.value
      }
    }
    return null
  }

  isLocal(): boolean {
    return !isRemoteProfile(this)
  }

}

interface UserCreateForm {
  username: string;
  password: string | null;
  message: string | null;
  signature: string | null;
  invite_code: string | null;
}

export async function createUser(
  loginType: AuthenticationMethod,
  userData: UserCreateForm,
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/accounts`
  const response = await http(url, {
    method: "POST",
    json: {
      authentication_method: loginType,
      ...userData,
    },
  })
  const data = await handleResponse(response, 201)
  return data
}

interface LoginForm {
  username: string | null;
  password: string | null;
  message: string | null;
  signature: string | null;
}

export async function getAccessToken(
  loginType: AuthenticationMethod,
  loginData: LoginForm,
): Promise<string> {
  const url = `${BACKEND_URL}/oauth/token`
  const tokenRequestData = {
    grant_type: loginType,
    ...loginData,
  }
  const response = await http(url, {
    method: "POST",
    json: tokenRequestData,
  })
  const data = await handleResponse(response)
  return data.access_token
}

export async function revokeAccessToken(
  authToken: string,
): Promise<void> {
  const url = `${BACKEND_URL}/oauth/revoke`
  const response = await http(url, {
    method: "POST",
    authToken,
    json: { token: authToken },
  })
  await handleResponse(response)
}

export async function getCurrentUser(authToken: string): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/accounts/verify_credentials`
  const response = await http(url, { authToken })
  const data = await handleResponse(response)
  return data
}

export async function lookupProfile(
  authToken: string | null,
  acct: string,
): Promise<Profile> {
  const url = `${BACKEND_URL}/api/v1/accounts/lookup`
  const response = await http(url, { authToken, queryParams: { acct } })
  const data = await handleResponse(response)
  return data
}

export async function getProfile(
  authToken: string | null,
  profileId: string,
): Promise<Profile> {
  const url = `${BACKEND_URL}/api/v1/accounts/${profileId}`
  const response = await http(url, { authToken })
  const data = await handleResponse(response)
  return data
}

export async function getProfiles(
  authToken: string,
  offset?: number,
): Promise<Profile[]> {
  const url = `${BACKEND_URL}/api/v1/directory`
  const queryParams = { offset, limit: PAGE_SIZE }
  const response = await http(url, { queryParams, authToken })
  const data = await handleResponse(response)
  return data
}

interface ProfileFieldAttrs {
  name: string;
  value: string;
}

export interface ProfileUpdateData {
  display_name: string | null;
  note: string | null;
  avatar: string | null;
  avatar_media_type: string | null;
  header: string | null;
  header_media_type: string | null;
  locked: boolean,
  mention_policy: "none" | "only_known" | "only_contacts",
  fields_attributes: ProfileFieldAttrs[];
}

export async function updateProfile(
  authToken: string,
  profileData: ProfileUpdateData,
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/accounts/update_credentials`
  const response = await http(url, {
    method: "PATCH",
    json: profileData,
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

interface UnsignedActivity {
  value: { [key: string]: any },
}

export async function getUnsignedUpdate(
  authToken: string,
): Promise<UnsignedActivity> {
  const url = `${BACKEND_URL}/api/v1/accounts/signed_update`
  const response = await http(url, { authToken })
  const data = await handleResponse(response)
  return data
}

export async function sendSignedActivity(
  authToken: string,
  activityValue: { [key: string]: any },
  signer: string,
  signatureHex: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/accounts/send_activity`
  const response = await http(url, {
    method: "POST",
    json: {
      value: activityValue,
      signer: signer,
      signature: signatureHex,
    },
    authToken,
  })
  await handleResponse(response)
}

export interface IdentityClaim {
  did: string,
  claim: string,
  created_at: string,
}

export async function getIdentityClaim(
  authToken: string,
  proofType: "ethereum" | "minisign" | "minisign-unhashed",
  signer: string,
): Promise<IdentityClaim> {
  const url = `${BACKEND_URL}/api/v1/accounts/identity_proof`
  const queryParams = { proof_type: proofType, signer }
  const response = await http(url, { authToken, queryParams })
  const data = await handleResponse(response)
  return data
}

export async function createIdentityProof(
  authToken: string,
  proofType: "ethereum" | "minisign" | "minisign-unhashed",
  did: string,
  signature: string,
  createdAt: string,
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/accounts/identity_proof`
  const response = await http(url, {
    method: "POST",
    json: {
      proof_type: proofType,
      did: did,
      signature: signature.replace(/^0x/, ""),
      created_at: createdAt,
    },
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

export interface Aliases {
  declared: Profile[],
  declared_all: { id: string, account: Profile | null }[],
  verified: Profile[],
}

export async function getAliases(profileId: string): Promise<Aliases> {
  const url = `${BACKEND_URL}/api/v1/accounts/${profileId}/aliases/all`
  const response = await http(url)
  const data = await handleResponse(response)
  return data
}

export async function loadLatestPosts(
  authToken: string,
  accountId: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/accounts/${accountId}/load_activities`
  const response = await http(url, {
    method: "POST",
    authToken,
  })
  await handleResponse(response, 204)
}
