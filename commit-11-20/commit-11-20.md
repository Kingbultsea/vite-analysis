# commit-11

添加测试注释（准备测试style HMR）。

# commit-12

注释。

```json
{
-   prepublishOnly: "tsc"
+   propublishOnly: "yarn build"    
}
```



# commit-13 v0.1.0发布

修改旧名称 ```vds```为```vite```，包括注释，控制台输出，只是更改名称。

# commit-14

`files` 字段用于描述我们使用 `npm publish` 命令后推送到 `npm` 服务器的文件列表，如果指定文件夹，则文件夹内的所有内容都会包含进来。我们可以查看下载的 `antd` 的 `package.json` 的`files` 字段，内容如下：

```json
"files": [
  "dist",
+ "bin"
],
```

因为```dist```是```build```的文件，```/bin/vite.js```是启动文件，所以这一部分发布到```npm```，提供用户使用即可。

### commit-15 v0.1.1发布

```json
{
-   version: "0.1.1"
+   version: "0.1.1" 
}
```

### commit-16 

```chore: readme```，修改```readme```。

### commit-17 添加ci

```yml
version: 2

defaults: &defaults
  docker:
    - image: vuejs/ci

step_restore_cache: &restore_cache
  restore_cache:
    keys:
    - v1-dependencies-{{ checksum "yarn.lock" }}-1
    - v1-dependencies-

step_install_deps: &install_deps
  run:
    name: Install Dependencies
    command: yarn --frozen-lockfile

step_save_cache: &save_cache
  save_cache:
    paths:
      - node_modules
      - ~/.cache/yarn
    key: v1-dependencies-{{ checksum "yarn.lock" }}-1

jobs:
  test:
    <<: *defaults
    steps:
      - checkout
      - *restore_cache
      - *install_deps
      - *save_cache
      - run: yarn test

workflows:
  version: 2
  ci:
    jobs:
      - test

```

