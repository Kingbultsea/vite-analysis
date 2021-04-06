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

### 关于中间件执行的路径

1. 检测路径是否为```__hmrClient```，通过他来建立客户端与服务端的```ws```链接。
2. 处理包含```.js```的路径，发送模块。
3. 处理包含```.vue```的路径，与前端组件相关。
4. ```koa2-history-api-fallback```。
5. ```koa-static```。