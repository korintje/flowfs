import { BACKEND_URL } from "@/constants"

import { handleResponse, http, PAGE_SIZE } from "./common"
import { CustomEmoji } from "./emojis"
import { Post } from "./posts"
import { Profile } from "./users"

interface EmojiReaction {
  content: string,
  emoji: CustomEmoji | null,
}

export interface Notification {
  id: string;
  type: string;
  account: Profile;
  status: Post | null;
  reaction: EmojiReaction | null,
  created_at: string;
}

export async function getNotifications(
  authToken: string,
  maxId?: string,
): Promise<Notification[]> {
  const url = `${BACKEND_URL}/api/v1/notifications`
  const queryParams = { max_id: maxId, limit: PAGE_SIZE }
  const response = await http(url, {
    method: "GET",
    queryParams,
    authToken,
  })
  const data = await handleResponse(response)
  return data
}
