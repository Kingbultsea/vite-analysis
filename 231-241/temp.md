# 231 - e78c9f7 修复外链import路径改写错误

不应该中断改写`import`流程，不然遇到外链后，不返回任何的`code`。

```typescript
if (isExternalUrl(id)) {
  break
}
```



# 232 - redame

### JSX

`.jsx` 和 `.tsx`也同样开箱即用， JSX 也是通过 `esbuild`来编译的。

因为 React 不提供 ES 模块构建，你可以使用 [es-react](https://github.com/lukejacksonn/es-react), 或者使用 Snowpack 将 React 预捆绑到 ES 模块中。 让它运行的最简单方法是:

```js
import { React, ReactDOM } from 'https://unpkg.com/es-react'

ReactDOM.render(<h1>Hello, what!</h1>, document.getElementById("app"));
```



# 233 - fcf709e changelog 

## [0.11.1](https://github.com/vuejs/vite/compare/v0.11.0...v0.11.1) (2020-05-06)

### Bug Fixes

- 修复外链import路径改写错误 ([e78c9f7](https://github.com/vuejs/vite/commit/e78c9f7680c2652b13f4270182c860417e388b2e))



# 234 - a135bfd v0.11.1

release v0.11.1