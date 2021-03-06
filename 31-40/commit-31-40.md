# 31 - c45e066 整合```css hmr```

把更新渲染```<style>```的代码，整合到```client```中。省去每次提取```<styele>```模块都需要经过服务器语法词汇分析的过程。



# 32 - ef95a00 http缓存与读取文件的缓存

### 添加的包

#### ```koa-conditional-get```:

Conditional Get 又名 条件式请求 ，常见实现有```Last-Modified``` 和 ```ETag``` 两种。

#### ```koa-etag```:

为```koa```的响应设置```etag-header```。

#### ```lru-cache```:

要搞清楚```LruCache``` 是什么之前，首先要知道 ```Android``` 的缓存策略。其实缓存策略很简单，举个例子，就是用户第一次使用网络加载一张图片后，下次加载这张图片的时候，并不会从网络加载，而是会从内存或者硬盘加载这张图片。

缓存策略分为添加、获取和删除，为什么需要删除缓存呢？因为每个设备都会有一定的容量限制，当容量满了的话就需要删除。

那什么是 ```LruCache```呢？其实```LRU(Least Recently Used)``` 的意思就是近期最少使用算法，它的核心思想就是会优先淘汰那些近期最少使用的缓存对象。

> 作者：一团捞面
> 链接：https://www.jianshu.com/p/e09870b60046

### ```cacheRead```

弃用```fs.readFile```，转为```cacheRead```。

```cacheRead```封装```fs.readFile```，读取到的文件与文件的上一次更新的时间戳，缓存在```LRUCache```中，下次读取文件的时候，首先在```LRUCache```中寻找。

```lastModified```: https://nodejs.org/api/fs.html#fs_stats_mtimems

```typescript
const moduleReadCache = new LRUCache<string, CacheEntry>({
  max: 10000
})

export async function cachedRead(path: string, encoding?: string) {
  const lastModified = (await fs.stat(path)).mtimeMs
  const cached = moduleReadCache.get(path)
  if (cached && cached.lastModified === lastModified) {
    return cached.content
  }
  console.log('reading from disk: ', path)
  const content = await fs.readFile(path, encoding)
  moduleReadCache.set(path, {
    content,
    lastModified
  })
  return content
}
```

### http缓存

使用中间件即可。

```typescript
app.use(require('koa-conditional-get')())
app.use(require('koa-etag')())
```



# 33 - f6ef1b1 进一步利用LRU

编译```.vue```文件的方法：

```parseSFC```、```compilerSFCMain```、``` compileSFCTemplate```与```compileSFCStyle```均把转换得出的结果保存在```vueCache```中。

```typescript
interface CacheEntry {
  descriptor?: SFCDescriptor // parseSFC
  template?: string // compilerSFCTemplate
  script?: string // compilerSFCMain
  styles: string[] // compileSFCStyle
}

export const vueCache = new LRUCache<string, CacheEntry>({
  max: 65535
})
```



# 34 - 052ac90 v0.3.0发布

### 我觉得这个版本能用，```<style>```的流程：

1. 获取```.vue```
2. 根据```parseSFC```，遍历```style```，每个子```style```均生成语句：```updateStyle("92a6df80-0", "/Comp.vue?type=style&index=0&t=1617780907326")```
3. ```clinet```端，调用```updateStyle```方法，创建出```<link id="vite-css-92a6df80-0" rel="stylesheet" type="text/css" href="/Comp.vue?type=style&amp;index=0&amp;t=1617780907326">```
4. ```server```端，接收到```/Comp.vue?type=style&amp;index=0&amp;t=1617780907326```
5. ```type```为```style```，```index```为```0```，```parseSFC```所编译的```AST```语法树```descriptor```，发送```descriptor.styles[index]```的内容给```client```端

### BUG：（```vue```的bug）

当新增```<style scoped>```，再去添加```class```样式不起效。
https://github.com/vuejs/vue-next/issues/3382



# 35 - 7b75253 304将不再处理内容

```modulesPlugin```中判断请求304，将不再处理内容。

```typescript
const internalPlugins: Plugin[] = [
  modulesPlugin,
  vuePlugin,
  hmrPlugin,
  servePlugin
]

# modulesPlugin的部分代码
app.use(async (ctx, next) => {
    await next()
    
    if (ctx.status === 304) {
      return
    }
})
```

根据洋葱模型，```modulesPlugin```判断```304```处于所有中间件执行的最后一个步骤。

# 36 - 0f88118 v0.3.1

v0.3.1



# 37 - 2c1b802 删除无用的包

删除```@babel/parser```



# 38 - 1e4a78c v0.3.2

v0.3.2

# 39 - 6e66766 构建```js-map```，```js```的```hmr```

目的：创建导入文件的关系图，在```hmr```的时候，可以知道应该热加载哪些文件。

### ```rewriteImports```

遇到```/^[^\/\.]/.test(id)```，改写成为```__modules/${id}```。

例如：
```import { ref } from 'vue'``` -> ```import { ref } from '__module/vue'```

如果遇到：

```import { a } from './vue'``` 不做处理，开头非```/```，非```.```的一律不做处理。

通俗来说，我们如何写node引入模块，就把我们引入的路径给改写。

### 构建```js```关系链

![cache](./A@B5FF6KI9XL_FKRY$U80CR.png)

目的是，收集所有需要```hmr```的```非模块文件```。

### ```hmr.ts```

使用```importeeMap```，获得关系链，进行```HMR```的一些东西。



# 40 - a183791 ```snowPack```

https://github.com/vitejs/vite/pull/4

详细可以去看这个```pr```了，支持```snowpack```。

### ```modules.ts```

```typescript
async function resolveWebModule(
  root: string,
  id: string
): Promise<string | undefined> {
  const webModulePath = webModulesMap.get(id)
  if (webModulePath) {
    return webModulePath
  }
  const importMapPath = path.join(root, 'web_modules', 'import-map.json')
  if (await fs.stat(importMapPath).catch((e) => false)) {
    const importMap = require(importMapPath)
    if (importMap.imports) {
      const webModulesDir = path.dirname(importMapPath)
      Object.entries(
        importMap.imports
      ).forEach(([key, val]: [string, string]) =>
        webModulesMap.set(key, path.join(webModulesDir, val))
      )
      return webModulesMap.get(id)
    }
  }
}
```

