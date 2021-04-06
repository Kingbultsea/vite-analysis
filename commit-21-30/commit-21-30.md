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

# commit-23

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

