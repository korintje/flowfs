declare const __APP_VERSION__: string

declare module "*.svg?component" {
  import { FunctionalComponent, SVGAttributes } from "vue"
  const src: FunctionalComponent<SVGAttributes>
  export default src
}
