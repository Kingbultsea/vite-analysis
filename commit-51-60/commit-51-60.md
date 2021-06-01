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

使用rollup打包

# commit-53

