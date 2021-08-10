# commit-71 模板

新增```create-vite-app```文件夹，可以通过命令`npx create-vite-app <project-name>`来创建。

![image-20210730051910514](./temp.png)

# commit-72 vite`为什么如此快？

## Getting Started

```bash
$ npx create-vite-app <project-name>
$ cd <project-name>
$ npm install
$ npm run dev
```

If using Yarn:

```bash
$ yarn create vite-app <project-name>
$ cd <project-name>
$ yarn
$ yarn dev
```

## 这与基于捆绑程序的设置有何不同?

主要区别在于，对于`vite`，在**开发过程中**没有捆绑。源代码中的**ES导入语法**直接提供给浏览器，浏览器通过`<script module>`支持和解析它们，为每次`import`发出HTTP请求。dev服务器拦截请求，并在必要时执行代码转换。例如，对`*.vue`文件的`import`，在发送给浏览器之前立即编译。

这种方法有几个优点:

- 由于没有捆绑工作要做，服务器冷启动极快
- 代码是按需编译的，所以只编译当前屏幕上实际导入的代码。你不必等到整个应用程序被捆绑后才能开始开发。这对于具有数十个屏幕的应用程序来说可能是一个巨大的差异
- 热模块更换 (HMR) 性能与模块总数分离。无论你的应用程序有多大，HMR 都始终快速

整页重新加载可能比基于捆绑器的设置稍慢，因为原生 ES `import`会导致一系列**深度`import`的网络请求**。然而，由于这是本地开发，与实际编译时间相比，差异应该是微不足道的。 （页面重新加载没有编译成本，因为已经编译的文件缓存在内存中。）

最后，因为编译在 `Node`环境 中完成，**它在技术上可以支持任何捆绑器进行代码转换**。事实上，`vite` 提供了一个 `vite build` 命令来做到这一点，因此应用程序在生产中不会受到**深度`import`的网络请求**的影响。

`vite` 在这个阶段是高度实验性的，不适合生产使用，但我们希望有一天能做到。

# commit-73 v0.5.2

release v0.5.2

# commit-74

```package.json```修正入口路径。

# commit-75 修复template与style同时hmr的BUG

修复`<template>`与`<style>`同时变动，触发hmr，只更新`<template>`的BUG。

顺带整理代码。

引起该`bug`的原因是`hmr`优先触发`vue-render`事件，触发后跳过`<style>`的`hmr`的处理了。

# commit-76 v0.5.3

发布`v0.5.3`

# commit-77 重构暴露路径处理器

`publicPath`: 浏览器请求资源路径。

`filePath`: 文件绝对路径。

> 在配置中暴露了一个`resolver`方法，尤大打算让处理文件不再写死。使用者可以修改`publicPath`与`filePath`

(未看见其使用，后续`commit`会继续分析)

# commit-78 添加http协商缓存

```typescript
export async function cachedRead(ctx: Context, file: string) {
  // 指示最后一次修改此文件的时间戳，以 POSIX Epoch 以来的毫秒数表示。
  const lastModified = (await fs.stat(file)).mtimeMs
  
  const cached = moduleReadCache.get(file)
  ctx.set('Cache-Control', 'no-cache')
  ctx.type = path.basename(file)
  if (cached && cached.lastModified === lastModified) {
    ctx.etag = cached.etag
    ctx.lastModified = new Date(cached.lastModified)
    ctx.status = 304
    return cached.content
  }
  const content = await fs.readFile(file, 'utf-8')
  const etag = getETag(content)
  moduleReadCache.set(file, {
    content,
    etag,
    lastModified
  })
  ctx.etag = etag
  ctx.lastModified = new Date(lastModified)
  ctx.body = content
  ctx.status = 200
}
```

添加缓存。

## http中etag和lastModified哪个优先级高

当ETag和Last-Modified同时存在时，服务器先会检查ETag，然后再检查Last-Modified，最终决定返回304还是200。

## 那为什么同时设置两个呢

https://segmentfault.com/q/1010000004200644。两者是and的关系。

ETag 比较的是响应内容的特征值，而Last-Modified 比较的是响应内容的修改时间。这两个是相辅相成的，并不是说有了ETag就不该有Last-Modified，有Last-Modified就不该有ETag。同时传入服务器时，服务器可以根据自己的缓存机制的需要，选择ETag或者是Last-Modified来做缓存判断的依据，甚至可以两个同时参考。

> `mtimeMs`的使用，可以查看commit- 82&83的解析，主要是后续重构把所有用到fs.read的地方都使用缓存，所以需要根据修改时间来判断是否使用缓存

# commit-79 fix测试的BUG

测试启动的时候，会判断是否子线程输出`running`。现在要同步修改成`Running`

```typescript
test('test', async () => {
  server = execa(path.resolve(__dirname, '../bin/vite.js'), {
    cwd: tempDir
  })
  await new Promise((resolve) => {
    server.stdout.on('data', (data) => {
      if (data.toString().match('Running')) {
        resolve()
      }
    })
  })
 })
```

# commit-80 fix处理模块的BUG

[#15](https://github.com/vitejs/vite/pull/15)

在vite中，所有源文件的`import Lodash from 'lodash'`模块类的都会被改写成`import Lodash from '@modules/lodash'`。

然后在发送该模块的时候，会使用正则去除`@modules`，得到`lodash`。

```typescript
let module = id = 'lodash'
const deepIndex = id.indexOf('/')

if (deepIndex > 0) {
    module = id.splice(0, deepIndex)
}

// ...
```

随后根据正则，获取到模块名称`module`。

但是如果引入的是：
`import { parse } from '@vue/compiler-dom`，我们得到的模块名称为`@vue`，那么就引起错误了。

原本加入`indexOf('/')`这些代码的目的是防止`lodash/index.js`的发生，实际上我们可以用`path.extname`判断是否有拓展名来过滤掉`/index.js`。

## 什么是scope module

所有npm模块都有name，有的模块的name还有scope。scope的命名规则和name差不多，同样不能有url非法字符或者下划线点符号开头。scope在模块name中使用时，以@开头，后边跟一个/ 。package.json中，name的写法如下：

> @somescope/somepackagename

scope是一种把相关的模块组织到一起的一种方式，也会在某些地方影响npm对模块的处理。

带有scope的模块安装在一个子目录中，如果正常的模块安装在node_modules/packagename目录下，那么带有scope的模块安装在node_modules/@myorg/packagename目录下，@myorg就是scope前面加上了@符号，一个scope中可以包含很多个模块。

安装一个带有scope的模块：

> npm install @myorg/mypackage

在package.json中写明一个依赖:

> "dependencies": {
> "@myorg/mypackage": "^1.3.0"
> }

如果@符号被省略，那么npm会尝试从github中安装模块，在npm install命令的文档中有说明 https://docs.npmjs.com/cli/install

