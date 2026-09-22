# AkiStash · Windows 桌面版

<p align="center">
  <a href="README.md"><b>English</b></a> | <a href="README_zh.md"><b>简体中文</b></a>
</p>

AkiStash 是一款二维 / 三维绘图软件。名字来自 Aki（制作者cn嘿嘿嘿）和仓鼠在秋天囤粮的习性：一个囤积颜色、笔刷和灵感的小仓库。

本目录是用 [Tauri 2](https://v2.tauri.app/) 打包的桌面版工程。完整的功能说明见 `功能说明.md`。

## 目录结构

```text
AkiStash/
├─ src/                      网页前端（离线可用）
│  ├─ index.html             软件本体
│  └─ three.min.js           三维引擎 Three.js r128（本地副本，不再依赖网络）
├─ src-tauri/                桌面外壳
│  ├─ src/main.rs、lib.rs    Rust 入口，注册对话框与文件插件
│  ├─ tauri.conf.json        应用名、窗口大小、图标、安装包设置
│  ├─ capabilities/          权限：只允许弹出保存对话框、写入用户选择的文件
│  └─ icons/                 应用图标（app-icon.png 是 1024×1024 的源图）
└─ .github/workflows/        在 GitHub 上自动打包的配置（可选）
```

## 第一次准备环境（只需一次）

1. **Rust**：已安装的话，在终端运行 `rustup update` 升级到最新稳定版。

2. **Microsoft C++ 生成工具**：安装 [Visual Studio 生成工具](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)，勾选“使用 C++ 的桌面开发”。如果装过 Visual Studio 并带了 C++ 工作负载，可以跳过。

3. **WebView2**：Windows 10/11 一般已经自带，不用管。

4. **Tauri 命令行工具**：
   ```bash
   cargo install tauri-cli --version "^2" --locked
   ```
   第一次编译需要几分钟。

## 开发运行

在 `AkiStash` 目录下打开终端：

```bash
cargo tauri dev
```

第一次会下载并编译依赖，时间比较长，之后就快了。会弹出一个 AkiStash 窗口，修改 `src/index.html` 后关掉重开即可看到效果。

## 打包安装程序

```bash
cargo tauri build
```

完成后安装包在：

- `src-tauri/target/release/bundle/nsis/AkiStash_1.0.0_x64-setup.exe`（推荐，中文安装界面）
- `src-tauri/target/release/bundle/msi/AkiStash_1.0.0_x64_en-US.msi`

把 `.exe` 发给别人双击安装即可。

## 桌面版与网页版的区别

- **完全离线**：三维引擎已放在本地；界面字体使用 Windows 自带的微软雅黑，不再从网上加载。
- **保存文件**：“保存”“导出 PNG”“导出笔刷”会弹出系统的“另存为”对话框。
- **打开文件、导入图片**：仍使用系统文件选择框；图片也可以直接拖进画布或按 Ctrl+V 粘贴。
- **防误刷新**：桌面版禁用了 F5 / Ctrl+R，避免未保存的作品丢失。

## 修改版本号、名字或图标

- **版本号**：同时改 `src-tauri/tauri.conf.json` 和 `src-tauri/Cargo.toml` 里的 `version`。
- **图标**：替换 `src-tauri/icons/app-icon.png` 后运行 `cargo tauri icon src-tauri/icons/app-icon.png`，会自动生成所有尺寸。

## 用 GitHub 自动打包（可选）

把整个目录推送到 GitHub 仓库后，打一个版本标签：

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions 会在云端的 Windows 机器上打包，并在仓库的 Releases 里生成一个草稿版本，附带安装包。确认无误后点“发布”即可。开源仓库使用 Actions 是免费的。

## 关于“Windows 已保护你的电脑”提示

安装包没有数字签名时，Windows SmartScreen 会拦截并提示“未知发布者”，点“更多信息 → 仍要运行”即可安装。正式发布时可以考虑：

- 申请 [SignPath Foundation](https://signpath.org/) 为开源项目提供的免费代码签名；
- 或者购买代码签名证书。

签名并积累一定下载量后，这个提示就会消失。

## 常见问题

- **编译报错找不到 `link.exe`**：说明没装 C++ 生成工具，见“第一次准备环境”第 2 步。
- **保存时报“没有权限”**：保存位置不在“用户文件夹 / 桌面 / 文档 / 下载 / 图片”之内。可以在 `src-tauri/capabilities/default.json` 的 `fs:scope` 里添加路径。

## 鸣谢

- 特别感谢**炸猫老师**给我提供了很多板绘软件常用的功能，以及帮我找出了套索的bug，绘制软件logo o(*￣▽￣*)ブ
- 感谢**rs老师**为我填补了软件提供工程制图方面的空白
- 感谢炸猫的老鼠作为logo出境（啊？）