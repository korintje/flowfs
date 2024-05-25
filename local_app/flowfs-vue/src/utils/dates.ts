import { DateTime } from "luxon"

export function humanizeDate(isoDate: string): string {
  const date = DateTime.fromISO(isoDate)
  const now = DateTime.now()
  const diff = now.diff(date)
  if (diff.as("minutes") < 60) {
    const minutes = Math.round(diff.as("minutes"))
    return `${minutes} minutes ago`
  } else if (diff.as("hours") < 24) {
    const hours = Math.round(diff.as("hours"))
    return `${hours} hours ago`
  } else if (diff.as("days") < 7) {
    const days = Math.round(diff.as("days"))
    return `${days} days ago`
  } else if (date.year === now.year) {
    return date.toFormat("dd LLL")
  } else {
    return date.toFormat("dd LLL y")
  }
}

export function formatDate(isoDate: string): string {
  const date = DateTime.fromISO(isoDate)
  return date.toLocaleString()
}

export function formatDateTime(isoDate: string): string {
  const date = DateTime.fromISO(isoDate)
  return date.toLocaleString(DateTime.DATETIME_FULL)
}

export function isPastDate(isoDate: string): boolean {
  const date = DateTime.fromISO(isoDate)
  return date < DateTime.now()
}
