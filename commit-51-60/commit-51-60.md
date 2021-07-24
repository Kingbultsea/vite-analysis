# commit-51 重构server路径

```server```端，名称统一改成```node```端，即更改路径/文件夹名称

新增了两个文件:

1. ```src/node/build.ts```

   无

2. ```src/node/index.ts```

   ```typescript
   export * from './server'
   ```

   

# commit-52 ```rollup```与```postcss```，打包

## ```package.json```

### ```postcss```

一个转换使用```js```来转换```css```的工具

### ```cssnano```

```postcss```插件，优化体积

### rollup

打包管理

## ```bin/vite.js```

错误处理，当端口被占用，抛出error，重新使用新的端口运行服务

## 新增```node/build.ts```

使用```rollup```进行打包。引入```rollup```的方式很特别，有效减少打包vite这个工具包的体积（如文字说，```import```的作用仅仅是当作```type```来使用，会被```treeShaking```掉，可以构建后查看```build.js```，```rollup```的引入会被去除）

``````typescri
function build() {
  // ...
  // lazy require rollup so that we don't load it when only using the dev server
  // importing it just for the types
  const rollup = require('rollup').rollup as typeof Rollup
}
``````

### 关于rollup

#### vitePlugin (rollup的插件)

```typescript
const vitePlugin: Plugin = {
    name: 'vite', // warning与error会被标识为该名称
    resolveId(id: string) { // 需要处理的id名称，如果返回Null，则load会正常运行，不做任何过滤
      if (id.startsWith('/')) {
        if (id === hmrClientPublicPath) {
          return hmrClientPublicPath
        } else {
          return id.startsWith(root) ? id : path.resolve(root, id.slice(1))
        }
      } else if (id === 'vue') {
        if (inlineVue) {
          return resolveVue(root, true).vue
        } else {
          return {
            id: cdnLink,
            external: true
              // 设置为true，将会被设置为相对路径
              // https://github.com/rollup/rollup/issues/3940
          }
        }
      }
    },
    // https://www.rollupjs.com/guide/plugin-development#load
    load(id: string) { // 返回sourcecode，同上，如果为Null，则不做处理
      if (id === hmrClientPublicPath) {
        return `export function hot() {}` // source code
      } else if (id === indexPath) {
        let script = ''
        let match
        
        // 收集index.html外部的js
        while ((match = scriptRE.exec(indexContent))) {
          // TODO handle <script type="module" src="..."/>
          // just add it as an import
          script += match[1]
        }
        return script
      }
    }
  }
```

目前没看出什么东西，对应的文件处理是```hmr```(本质就是```client/client.ts```)与```index.html```



### cssExtractPlugin

收集所有```styles code```，粘连所有```styles code```，生成一个```css```文件。

需要放在处理SFC的Plugins的后面。

```typescript
const cssExtractPlugin: Plugin = {
    name: 'vite-css',
    transform(code: string, id: string) {
      if (id.endsWith('.css')) {
        styles.set(id, code)
        return '/* css extracted by vite */'
      }
    }
  }
```

### rollup-plugin-vue

处理SFC（在```dev```环境，我们可以借助```@vue/compiler-sfc```，来处理）。

有兴趣的可以看这里：

https://github.com/vuejs/rollup-plugin-vue/blob/next/src/index.ts

和我们```dev```同一个作用。



### @rollup/plugin-node-resolve

处理Node的第三方包。



### @rollup/plugin-replace

打包的时候，修改字符串。

```typescript
require('@rollup/plugin-replace')({
        'process.env.NODE_ENV': '"production"'
})
```



### rollup-plugin-terser

压缩代码，放在最后用（就是plugins数组的最后一个）。

```typescript
// 类型
type Last<T extends any[]> = [never, ...T][T['length']]
```



### rollup配置options

```typescript
{
    // TODO
    // parse index.html
    // find entry file or script
    // if inline script, create a temp main file next to it before bundling
    input: path.resolve(root, 'index.html'),
    plugins: [
      vitePlugin,
      require('rollup-plugin-vue')(),
      require('@rollup/plugin-node-resolve')({
        rootDir: root
      }),
      require('@rollup/plugin-replace')({
        'process.env.NODE_ENV': '"production"'
      }),
      cssExtractPlugin,
      require('rollup-plugin-terser').terser()
    ]
  }
```

```typescript
const { output } = await bundle.generate({
    dir: outDir,
    format: 'es'
})
```



### 过程(未完善)

未完善的原因是```vitePlugin```中的```load```方法并未处理```src```资源，仅处理了标签内的内容
```<script src="main.js">console.log(1)</script>```

```typescript
// vitePlugins中的load方法

let script = ''
et match
console.log(indexContent)
while ((match = scriptRE.exec(indexContent))) {
  // TODO handle <script type="module" src="..."/>
  // just add it as an import
  script += match[1]
}
return script
```

1. 配置好```rollup```，删除旧的文件夹后，创建出新的空的文件夹(生成代码的位置)，这个步骤是，vite需要再对输出的文件改写。

2. 植入```css```文件到```html文件模板```。

   ```typescript
   `<link rel="stylesheet" href="/${filename}">`
   // 注意是link
   ```

3. ```rollup```打包出来的```css```，全部字符串堆起来，交给```postcss```与```postcss```的插件处理，并生成css文件（等于改写）。

4. 检测有没有本地vue包，没有则cdn```https://unpkg.com/vue@${vueVersion}/dist/vue.esm-browser.prod.js```，以```<script src="" >```的形式

5. rollup打包出来的js，不改写，但是在```html```模板植入```<script/>```标签。

6. 最后，根据```html模板```（带```<link/> <script/>```），生成```html```文件。



## node/resolveVue.ts

根据打包环境，如果运行在浏览器，则使用```esm-browser```包，如果在node环境，则使用```esm-bundler```，后者不带编译器。

# commit-53

修改测试的命令，路径为src。

## 再细谈文件改动

```typescript
watcher.on('change', async (file) => {
    const timestamp = Date.now()
    const servedPath = '/' + path.relative(root, file)
    if (file.endsWith('.vue')) {
        // 处理vue文件
      handleVueSFCReload(file, servedPath, timestamp)
    } else {
        // 处理js文件
      handleJSReload(servedPath, timestamp)
    }
  })
```

### ```handleVueSFCReload```

1. 根据文件路径，询问```vueCache```这个```Set```是否有缓存，有则返回缓存```vue```文件（该文件正是```SFC组件```被```vuePlugins```处理好的```js```文件，如果不懂，可以理解为使用```vue-loader```转换```A.vue```组件为```js版本```的组件）。这里取缓存的作用是来分析```SFC```哪块标签变动，准确发送事件，性能优化。

2. 因为文件已经改动了，所以删除缓存结果```vueCache.del(file)```

3. ```parseSFC```，读取```vueCache```缓存(没有缓存了，被上一步删除了，在**文件改动中**无效，已验证)，```SFC```转换成```js```组件处理

4. ```parseSFC```对于一些有BUG的```SFC组件```，无法处理，那么就不会返回```descriptor```这个字段，就是说流程结束。

5. 如果```SFC组件```被成功转换成```js组件```：
   ```<script>```变动：发送```vue-reload事件```（**更新字段，通知父组件重新渲染vnode**，```instance.update```，如忘了这些事件，请查看**commit-6**的```reload事件```）

   ```typescript
   import { updateStyle } from "/@hmr"
   
   import { render as __render } from "/Child.vue?type=template&t=1627148468989&t=1627148468989"
   __script.render = __render
   __script.__scopeId = "data-v-92a6df80"
   __script.__hmrId = "/Child.vue"
   __script.__file = "E:\\vite\\test\\fixtures\\Child.vue"
   export default __script
   
   // 更新的字段就是render __hmrId __file
   ```

   ```<template>```变动：发送```vue-rerender```事件（**重新渲染自身组件vnode**，```instance.update```，[commit-6-rerender](https://github.com/Kingbultsea/vite-analysis/blob/a1a7c85a55909ac4457dc2fe40f3eebdccec2ad1/readme.md#%E5%85%B3%E4%BA%8Ererender%E4%BA%8B%E4%BB%B6)）

   ```<style>```变动：
     如果标签内的属性```scoped```变动，发送```vue-reload```事件，去除```js组件```的```__scopeId```;
     如果内容改变，则发送```vue-style-update```事件；

     ```typescript
     function updateStyle(id: string, url: string) {
       const linkId = `vite-css-${id}`
       let link = document.getElementById(linkId)
       if (!link) {
         link = document.createElement('link')
         link.id = linkId
         link.setAttribute('rel', 'stylesheet')
         link.setAttribute('type', 'text/css')
         document.head.appendChild(link)
       }
       link.setAttribute('href', url)
     }
     ```

   如果旧的```<styles/>```标签数量多于新的，则发送```vue-style-remove```事件；

   ```typescript
   case 'vue-style-remove':
         const link = document.getElementById(`vite-css-${id}`)
         if (link) {
           document.head.removeChild(link)
         }
         break
   ```

### ```handleJSReload``` // todo

1. 分析```importerMap```，取出所有```import```了**当前文件的路径**。
2. 如果引入方为```vue```文件，触发```vue-reload```

todo

# commit-54

更新rollup-plugin-vue包

```json
{
-    “rollup-pkugin-vue”: "6.0.0-alpha.1"
+    “rollup-pkugin-vue”: "6.0.0-alpha.1"
}
```

