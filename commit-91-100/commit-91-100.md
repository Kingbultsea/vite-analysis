# commit-91 修改字符

`util.ts`中的`readCache`方法里面写错`etag`字母（原tag）。

# commit-92 缓存304

我在commit-84提过，为什么在`serverPluginVue.ts`中缓存不设置304，现在补充上。

# commit-93 代码整理

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

# commit-94 调整LRU值(降低)

`rewriteCache`(涉及`index.html`与所有被改写`import`语句的文件)大小从`65535`改小成`1024`。

（话说我去看`lru-cache`的文档，也不知道这个max对应的单位是多少...，如果有知道的童鞋请告诉我）

# commit-95 设置node运行版本

指定项目运行node版本的范围。

> *engines*属性仅起到一个说明的*作用*，当用户版本不符合指定值时也不影响依赖的安装。

```json
"engines": {
    "node": ">=10.0.0"
}
```

# commit-96 v0.6.1

release v0.6.1

