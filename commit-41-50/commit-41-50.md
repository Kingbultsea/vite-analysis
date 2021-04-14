# commit-41 修改特殊路径名称

处理特殊路径```__```改为```@```

### hmr.ts

```typescript
- if(ctx.path !== '/__hmrClient')
+ if(ctx.path !== '/@hmr')
```

#### 关于js文件的重加载

在commit-39的时候，分析不出其行为，现在根据修改后的代码得出：

1. ```importerMap```查看是否有该```文件路径```的值。

2. 有，则取出其完整路径```importee```。

