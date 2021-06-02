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

使用```rollup```打包，引入```rollup```的方式很特别。通过这种方式，有效减少打包vite这个工具包的体积（如文字说，```import```的作用仅仅是当作```type```来使用，会被```treeShaking```掉，可以构建后查看```build.js```，```rollup```的引入会被去除）

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
    resolveId(id: string) { // 需要处理的id名称，如果返回Null，则不做处理
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
          }
        }
      }
    },
    load(id: string) { // 返回sourcecode，同上，如果为Null，则不做处理
      if (id === hmrClientPublicPath) {
        return `export function hot() {}` // source code
      } else if (id === indexPath) {
        let script = ''
        let match
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

打包的时候，修改字符串。把修改Node的全局变量。

```typescript
require('@rollup/plugin-replace')({
        'process.env.NODE_ENV': '"production"'
})
```



### rollup-plugin-terser

压缩代码，最好放在最后用（就是plugins数组的最后一个）。

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



### 过程

1. 配置好```rollup```，删除旧的文件夹后，创建出新的空的文件夹(生成代码的位置)，这个步骤是，vite需要再对输出的文件改写。

2. 植入```css```文件到```html文件模板```。

   ```typescript
   `<link rel="stylesheet" href="/${filename}">`
   // 注意是link
   ```

3. rollup打包出来的css，全部字符串堆起来，交给postcss与postcss的插件处理，并生成css文件（等于改写）。
4. rollup打包出来的js，不该写，但是在html模板植入script的应用，同步骤3。
5. 最后，根据```html模板```（带```<link/> <script/>```），生成html文件。



## node/resolveVue.ts

根据打包环境，如果运行在浏览器，则使用```esm-browser```包，如果在node环境，则使用```esm-bundler```，后者不带编译器。

# commit-53

