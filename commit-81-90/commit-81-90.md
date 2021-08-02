# commit-81 暴露watchAPI

```typescript
export type ViteWatcher = FSWatcher & {
  handleVueReload: (file: string, timestamp: number, content?: string) => void
  handleJSReload: (file: string, timestamp: number) => void
}
  
interface PluginContext {
  root: string
  app: Koa
  server: Server
  watcher: ViteWatcher
  resolver: InternalResolver
}
```

```typescript
# serverPluginHmr
watcher.handleVueReload = handleVueReload
watcher.handleJSReload = handleJSReload
```

`vue-plugins`上下文新增`watcher`。

也就是说plugins现在可以操控`vue`或者`js`文件的`hmr`了。

# commit-82&commit-83

代码整理，把所有读取文件的地方都使用`cacheRead`方法来读取。 

此处还有`ctx.vue`的相关代码(判断是否是vue文件)，但是还没有看到定义，只有其使用。

## 如果把所有文件都缓存了起来，hmr岂不是无效？

> 文件缓存的时候，会读取`(await fs.stat(file)).mtimeMs`修改时间，下次文件改变，将对比`mtimeMs`来决定是否使用缓存

# commit-84 为vue文件添加http缓存

**`If-None-Match`** 是一个条件式请求首部。对于 GET[`GET`](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Methods/GET) 和 [`HEAD`](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Methods/HEAD) 请求方法来说，当且仅当服务器上没有任何资源的 [`ETag`](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Headers/ETag) 属性值与这个首部中列出的相匹配的时候，服务器端会才返回所请求的资源，响应码为 [`200`](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Status/200)。

当你**第一次**发起HTTP请求时，**服务器**会返回一个**Etag**。

并在你**第二次**发起**同一个请求**时，客户端会**同时**发送一个**If-None-Match**，而它的值就是**Etag**的值（此处由发起请求的客户端来设置）。

然后，**服务器会比**对这个客服端发送过来的Etag**是否与服务器的相同**，如果**相同**，就将**If-None-Match**的值设为**false**，返回状态为**304，****客户端**继续**使用本地缓存**，不解析服务器返回的数据。

请求`vue组件`，判断request请求的`If-None-Match`是否与文件`etag`一致，一致则返回`ctx.status = 200`

## 之前不是已经设置过了缓存了吗？

首先说明一下，封装的`cachedRead`只对文件来说适用，对于`vue组件`来说，是需要编译的。

为了解决这个问题，尤大不再使用`cacheRead`对于`vue`文件，而是在编译了三大标签（`<style>` `<template>` `<script>`）后使用`etagCacheCheck`：

```typescript
const etagCacheCheck = (ctx: Context) => {
  ctx.etag = getEtag(ctx.body)
  if (ctx.etag !== ctx.get('If-None-Match')) {
    ctx.status = 200
  }
    // 好奇 为什么不else后返回304? 不然一样无效的 可能是因为想调试方便吧，浏览器点击就弹出源码了，不用查看sources
}
```

# commit-85 添加缓存

为访问`web_modules`使用`cacheRead`。

为访问`node_modules`的文件路径结果添加上缓存，不使用`cacheRead`的原因是跳转:

```typescript
# serverPluginModules

// resolve from web_modules
    try {
      const webModulePath = await resolveWebModule(root, id)
      if (webModulePath) {
        idToFileMap.set(id, webModulePath)
        await cachedRead(ctx, webModulePath)
        debugModuleResolution(
          `web_modules: ${id} -> ${getDebugPath(webModulePath)}`
        )
        return
      }
    } catch (e) {
      console.error(
        chalk.red(`[vite] Error while resolving web_modules with id "${id}":`)
      )
      console.error(e)
      ctx.status = 404
    }

    // resolve from node_modules
    try {
      let pkgPath
      try {
        pkgPath = resolve(root, `${id}/package.json`)
      } catch (e) {}
      if (pkgPath) {
        const pkg = require(pkgPath)
        const entryPoint = pkg.module || pkg.main || 'index.js'
        debugModuleResolution(`node_modules entry: ${id} -> ${entryPoint}`)
        idToEntryMap.set(id, entryPoint)
        return ctx.redirect(path.join(ctx.path, entryPoint))
      }
      // in case of deep imports like 'foo/dist/bar.js'
      const modulePath = resolve(root, id)
      idToFileMap.set(id, modulePath)
      debugModuleResolution(
        `node_modules import: ${id} -> ${getDebugPath(modulePath)}`
      )
      await cachedRead(ctx, modulePath)
    } catch (e) {
      console.error(
        chalk.red(`[vite] Error while resolving node_modules with id "${id}":`)
      )
      console.error(e)
      ctx.status = 404
    }
```

#### 可以利用ctx.redirt跳转

可以使用`return ctx.redirect(path.join(ctx.path, cachedEntry))`的方法，更改请求`path`路径（对于服务端来说，对于一整个`http`是没有影响的）。

# commit-86

为readCache方法设置`If-None-Match`。

```typescript
# node/util
function readCache() {
    // ... 
    if (ctx.get('If-None-Match') === ctx.tag) {
        ctx.status = 304
    }
    // ...
}

```

# commit-87 修复windows hmr问题

新增slash包。

## slash包

`resolver.ts`中的`defaultFileToPublic`（转换文件路径到浏览器请求路径）方法，使用`slash`统一转换成`/`

```typescript
import path from 'path';
import slash from 'slash';

const string = path.join('foo', 'bar');
// Unix    => foo/bar
// Windows => foo\\bar

slash(string);
// Unix    => foo/bar
// Windows => foo/bar
```

# commit-88 v0.6.0

release v0.6.0

