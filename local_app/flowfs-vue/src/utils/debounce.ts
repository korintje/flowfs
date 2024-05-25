export function debounce(
  func: (...args: unknown[]) => void,
  delay = 100,
): (...args: unknown[]) => void {
  let timeout: ReturnType<typeof setTimeout>
  return (...args: unknown[]) => {
    clearTimeout(timeout)
    timeout = setTimeout(() => { func.apply(this, args) }, delay)
  }
}
