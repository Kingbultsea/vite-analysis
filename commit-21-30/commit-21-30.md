# commit-21 v0.1.2发布

```json
{
-   version: "0.1.1"
+   version: "0.1.2"    
}
```

# commit-22 重构使用koa，废弃server-handler

均使用koa中间件的形式改写```vueMiddleware.ts moduleMiddleware.ts hmrWatcher.ts```

### server/middlewares/serve.ts

使用```koa2-history-api-fallback```，与```koa-static```中间件。

#### ```koa2-history-api-fallback```:

使用koa2-connect-history-api-fallback之后，koa就会把所有的get方式的请求都发给/index.html,然后由vue-router来接管页面路由。

#### ```koa-static```:

```typescript
app.use(require('koa-static')(cwd))
```

获取当前```cwd```路径的静态资源。

### 关于中间件执行的路径处理

1. 检测路径是否为```__hmrClient```，通过他来建立客户端与服务端的```ws```链接。
2. 处理包含```.js```的路径，发送模块。
3. 处理包含```.vue```的路径，与前端组件相关。
4. ```koa2-history-api-fallback```。
5. ```koa-static```。

# commit-23 index.html的指向

去除```koa```的中间件```koa2-history-api-fallback```，采用手写的方法```src/server/middlewares/historyFallback.ts```。

```typescript
import { Middleware } from '../index'

export const historyFallbackMiddleware: Middleware = ({ cwd, app }) => {
  app.use((ctx, next) => {
    const cleanUrl = ctx.url.split('?')[0].split('#')[0]
    if (ctx.method !== 'GET' || cleanUrl.includes('.')) { // 文件 get 不处理
      return next()
    }

    if (!ctx.headers || typeof ctx.headers.accept !== 'string') { // 没有header  || 不知道
      return next()
    }

    if (ctx.headers.accept.includes('application/json')) { // 不处理 期望json的数据
      return next()
    }

    if (
      !(
        ctx.headers.accept.includes('text/html') ||
        ctx.headers.accept.includes('*/*')
      ) // 边缘处理
    ) {
      return next()
    }

    ctx.url = '/index.html' // 改写路径，交给第5步，处理内容。
    return next()
  })
}

```

因为使用```koa2-history-api-fallback```，会把所有get请求都指向一个文件，如果请求一个```.vue```组件，在进行流程的第```4```步，内容必然会被改写成```index.html```文件。

# commit-24 改写```index.html```的```<script>```

曾经```index.html```请求页面，页面中的标签，引入```main.js```，经过```moduleRewriter.ts```处理，```import a from 'a'```改写成```import a from '__module/a'```。

现在把这个功能也用在浏览器请求```index.html```中的```<script>```。

优化目的提前一步改写处理。

# commit-25

### 更改名称:

```vueResole.ts``` -> ```resolveVue.ts``

### 优化改写```import```路径的代码结构

去除```moduleRewriter.ts```，并且把该```rewrite```功能合并到```middlewares/modules.ts```，由于这个用来改写```import```的，所以名称更改为```rewriteImports```。

好处是：原本在vue中间件，模块中间件，都需要指向```__modules```的功能，移交给```modules.ts```统一处理，

通过中间件，好管理代码。

# commit-26 v0.2.0发布

```json
{
-   version: "0.1.2"
+   version: "0.2.0"    
}
```

# commit-27 准备```vite```配置

### ```build```前需要删除dist: 

```json
{
    script: {
        // 不支持windows
        build: "rm -rf dist && tsc -p src/client && tsc -p src/server"
    }
}
```

### ```server/index.ts```

暴露可配置的server，交付给```./bin/vite.js```去建立服务，同时添加```https```选项。

启动```vite```，以命令行的形式去输入配置（这块有BUG，还没完善）

# commit-28 ```cwd```名称优化

```cwd```参数名称改为```root```，尤大觉得因为寻找模式，直接改为```root```会更加贴切（我觉得尤大觉得）。

