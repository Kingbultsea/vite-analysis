# commit-61 readme

新增build参数配置（但是未实现）

- `vite build --root dir`:  在目标目录而不是当前工作目录中生成文件

- `vite build --cdn`:  从CDN中引入```vue```，这样可以使构建速度更快，但总的来说，页面负载将更大，因为```VUE API```不会被```tree-shaking```。

# commit-62 v0.5.1

release v0.5.1

# commit-63 readme

补充```npx vite```，```npx```会自动把```vite```下载在```npm```缓存中。（使用后会自动删除，所以不用担心无法拉取最新的```vite```的问题）

>除了调用项目内部模块，```npx``` 还能避免全局安装的模块。比如，````create-react-app````这个模块是全局安装，```npx``` 可以运行它，而且不进行全局安装。
>
>> ```bash
>> $ npx create-react-app my-react-app
>> ```
>
>上面代码运行时，```npx``` 将`create-react-app`下载到一个临时目录，使用以后再删除。所以，以后再次执行上面的命令，会重新下载`create-react-app`。
>
>[原文](https://www.ruanyifeng.com/blog/2019/02/npx.html)

# commit-64 修改readme的单词错误

[fix#10](https://github.com/vitejs/vite/pull/10)，说起来也好笑... 混一个```vite```的```contributor```也太容易了吧。

# commit-65 ```vite```服务添加```ip```提醒

[feat #8](https://github.com/vitejs/vite/pull/8)，在使用```vite```开启服务的时候，我们可能不想使用```localhost```，想使用```ip```的方式。

# commit-66 readme

尤大觉得```vite```很快，就在```readme```文档大标题，加了一个⚡符号。

# commit-67 fix ```style```的```hmr```问题

在```commit-67```之前，```module```的改变，不会触发```hmr```，现在暂时触发```vue-reload```事件，快速修复问题为主，我们在做业务出现```bug```的时候，可以借鉴用简单的方法先解决问题，用注释去标记正确做法。

```typescript
if (
      prevStyles.some((s) => s.scoped) !== nextStyles.some((s) => s.scoped) ||
      // TODO for now we force the component to reload on <style module> change
      // but this should be optimizable to replace the __cssMoudles object
      // on script and only trigger a rerender.
    
    // 我们先强制触发vue-reload事件
    // 实际上我们可以优化成打包出来的js组件去除__cssMoudles对象，并重新触发组件render
    
      prevStyles.some((s) => s.module != null) ||
      nextStyles.some((s) => s.module != null)
    ) {
      notify({
        type: 'vue-reload',
        path: servedPath,
        timestamp
      })
    }
```

# commit-68 127.0.0.1修改为localhost

在[fix#10](https://github.com/vitejs/vite/pull/10)中，优化```localhost```成为```ip```地址，但是我们可以把```127.0.0.1```输出的提示替换为```localhost```

# commit-69 readme英语语法错误

[fix#12](https://github.com/vitejs/vite/pull/12)，英文语法错误。

# commit-70

## 实现```vite build --cdn```

判断是否设置了```--cdn```，传入```vite```的构建方法入口。

在build方法执行后，设置```process.env.NODE_ENV = 'production'```

## rollup配置```preserveEntrySignatures```为false

[文档](https://rollupjs.org/guide/en/#preserveentrysignatures)

```false```: 不会将入口模块的任何```export```添加到相应的块中，甚至不包含相应的代码，除非在包中的其他地方使用。但是，内部导出可以添加到条目块中。这是 Web 应用程序的推荐设置，其中条目块将放置在脚本标记中，因为它可能会减少块的数量和包大小。

**Example**
Input:

```
// main.js
import { shared } from './lib.js';
export const value = `value: ${shared}`;
import('./dynamic.js');

// lib.js
export const shared = 'shared';

// dynamic.js
import { shared } from './lib.js';
console.log(shared);
```

Output for `preserveEntrySignatures: false`

```
// main.js
import('./dynamic-39821cef.js');

// dynamic-39821cef.js
const shared = 'shared';

console.log(shared);
```

简单说明就是不使用```export```，如果```import```引入了```export```的内容，那么会在使用```import```的文件中，复制该内容。

