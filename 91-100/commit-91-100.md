# 91 - 43fe56f 修改字符

`util.ts`中的`readCache`方法里面写错`etag`字母（原tag）。



# 92 - 875c0c6 缓存304

我在commit-84提过，为什么在`serverPluginVue.ts`中缓存不设置304，现在补充上。



# 93 - 02b68d5 代码整理

整理`commit-92`的代码。

```typescript
// old
if (ctx.etag !== ctx.get('If-None-Match')) {
    ctx.status = 200
} else {
    ctx.status = 304
}

// new
ctx.status = ctx.etag === ctx.get('If-None-Match') ? 304 : 200
```



# 94 - 488fec3 调整LRU值(降低)

`rewriteCache`(涉及`index.html`与所有被改写`import`语句的文件)大小从`65535`改小成`1024`。

（话说我去看`lru-cache`的文档，也不知道这个max对应的单位是多少...，如果有知道的童鞋请告诉我）



# 95 - aceb5f7 设置node运行版本

指定项目运行node版本的范围。

> *engines*属性仅起到一个说明的*作用*，当用户版本不符合指定值时也不影响依赖的安装。

```json
"engines": {
    "node": ">=10.0.0"
}
```



# 96 - c3d87db v0.6.1

release v0.6.1



# 97 - df07231 介绍v0.6.1变动

### Bug Fixes

- 改写`import语句`的功能，应该在缓存304的时候不触发 ([c3a7a96](https://github.com/vuejs/vite/commit/c3a7a967ee9048ca6fc2642b3494b0e60978bf24))
- tag -> etag ([43fe56f](https://github.com/vuejs/vite/commit/43fe56f61b3f5cd8fc51d33916d79e154042bc8c))

我真觉得那个`import语句`的304没有任何意义（详情看commit-90）。

# 98 - 5794291 优化

对于已经被处理过的请求，不再经过`serverPluginServe`。

```typescript
app.use((ctx, next) => {
    if (ctx.body || ctx.status !== 404) {
      return
    }
    return next()
})

// 使用到的koa插件...
// https://www.npmjs.com/package/koa-etag
app.use(require('koa-conditional-get')()) // 实现基于etag的缓存
app.use(require('koa-etag')()) // 配合koa-conditional-get使用
app.use(require('koa-static')(root)) // 静态文件处理
```

> serverPluginServe用于处理一些静态文件请求与一些header服务



# 99 - d00523f 自动添加import文件后缀

那个没有用的304已经被去除了，正如`commit-90`所说的一样。

## 对于没有添加尾缀的import默认指向`js`文件

重写`import`语句的时候，如果没有添加文件后缀则：

```typescript
if (!/\.\w+/.test(pathname)) {
    pathname += '.js'
}
```



# 100 - 8965b65 修改resolver的名称

曾经`public`为浏览器请求使用，现在把`public`名称更改为`request`更符合语义了。



