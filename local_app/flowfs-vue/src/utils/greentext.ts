export function addGreentext(text: string): string {
  // Safari doesn't support lookbehind expressions
  try {
    /* eslint-disable-next-line prefer-regex-literals */
    const greentextRegexp = new RegExp("(?<=^|>)(&gt;[^<]+)(?=$|<)", "gm")
    return text.replace(greentextRegexp, '<span class="greentext">$1</span>')
  } catch (error) {
    return text
  }
}
