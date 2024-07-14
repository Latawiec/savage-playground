/* eslint-disable */
declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  import type { App } from 'vue'
  const component: DefineComponent<{}, {}, any>
  const app: App<any>
  export default component
  export default app
}
