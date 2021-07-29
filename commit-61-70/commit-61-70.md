# commit-61 readme

新增build参数配置（但是未实现）

- `vite build --root dir`:  在目标目录而不是当前工作目录中生成文件

- `vite build --cdn`:  从CDN中引入```vue```，这样可以使构建速度更快，但总的来说，页面负载将更大，因为```VUE API```不会被```tree-shaking```。

