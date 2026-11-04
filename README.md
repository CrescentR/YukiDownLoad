<div align="center">

<img src="src-tauri/icons/128x128@2x.png" width="128" height="128" alt="Yuki logo" />

# Yuki · 雪迹

**轻量、专注、跨平台的桌面视频下载器**
<br/>
先解析，再选择，后下载 —— 基于 Tauri 2 + Rust，由 yt-dlp 与 FFmpeg 驱动

[![Release](https://img.shields.io/github/v/release/CrescentR/YukiDownLoad?include_prereleases&style=flat-square&color=6c8cff&label=release)](https://github.com/CrescentR/YukiDownLoad/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/CrescentR/YukiDownLoad/yuki-ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/CrescentR/YukiDownLoad/actions/workflows/yuki-ci.yml)
[![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-8a8f98?style=flat-square)](#-下载安装)
[![Tauri](https://img.shields.io/badge/Tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-stable-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Unlicense-4caf50?style=flat-square)](LICENSE)

**简体中文** · [English](README_EN.md)

[下载安装](#-下载安装) · [快速上手](#-快速上手) · [使用教程](#-使用教程) · [常见问题](#-常见问题) · [开发构建](#️-开发与构建)

</div>

---

## ✨ 简介

**Yuki（雪迹）** 是一个围绕 [yt-dlp](https://github.com/yt-dlp/yt-dlp) 与 [FFmpeg](https://ffmpeg.org) 打造的原生桌面下载器。粘贴链接后，Yuki 会先读取视频的**真实**标题、封面、时长和完整清晰度列表，让你看清每条轨道的分辨率、编码、帧率和预估体积，再决定下载什么。

- 🪶 **轻巧**：前端为 Tailwind CSS 4 + 原生 ES Modules，没有重型 UI 框架；下载队列、进程监管与依赖托管全部由 Rust 实现。
- 📦 **开箱即用**：无需预装 Python，无需管理员权限；首次启动可在应用内一键下载并校验 yt-dlp 与 FFmpeg。
- 🌐 **站点广泛**：继承 yt-dlp 的能力，支持 YouTube、哔哩哔哩、抖音等 1000+ 站点。
- 🎨 **精致**：深色 / 浅色双主题，简体中文 / English 即时切换，本地自托管字体。

## 🧩 功能特性

<table>
<tr>
<td width="50%" valign="top">

### 🔍 下载前解析
展示真实格式列表（`format_id`、编码、帧率、HDR、预估体积），优先推荐直连 HTTPS 流；选中视频轨时自动匹配最佳音频合并。

</td>
<td width="50%" valign="top">

### 📚 批量与播放列表
多行批量粘贴（每行一个链接）；整张播放列表下载，或用 `1-10,15` 这样的区间语法挑选条目。

</td>
</tr>
<tr>
<td valign="top">

### 🍪 多站点 Cookie
从 Chrome / Edge / Firefox / Brave / Opera / Vivaldi / Safari 读取，导入 `cookies.txt`，或直接粘贴请求头中的 Cookie —— 无需制作文件。

</td>
<td valign="top">

### ⏯️ 队列与断点续传
可配置并发数，超出自动排队；暂停保留分片、继续时续传；取消任务会连同 `ffmpeg` 子进程树一起清理。

</td>
</tr>
<tr>
<td valign="top">

### 🌍 代理与传输控制
HTTP / HTTPS / SOCKS5 代理（下载与依赖更新共用），限速、分片并发数、重试次数均可配置。

</td>
<td valign="top">

### 🩺 实时日志与诊断
任务详情抽屉显示实时日志与脱敏后的完整命令行；常见错误（未登录、403、缺少 FFmpeg / JS 运行时、超时）会给出可操作的修复建议。

</td>
</tr>
<tr>
<td valign="top">

### 🛡️ 运行时托管
yt-dlp 与 FFmpeg 下载全程 SHA-256 校验、试运行、失败回滚；在应用内检查并更新到上游最新版本。

</td>
<td valign="top">

### 🎵 音频与附加内容
提取 FLAC / MP3 / M4A / Opus 音频；写入元数据、嵌入封面与字幕（含自动字幕），可保存 `info.json`。内置抖音兼容插件。

</td>
</tr>
</table>

## 📥 下载安装

前往 **[Releases 页面](https://github.com/CrescentR/YukiDownLoad/releases)** 下载对应平台的安装包：

| 平台 | 安装包 | 说明 |
| --- | --- | --- |
| 🪟 **Windows** 10 / 11（x64） | `Yuki_x.y.z_x64-setup.exe` | NSIS 安装程序，安装到当前用户，无需管理员权限 |
| 🍎 **macOS**（Apple Silicon） | `Yuki_x.y.z_aarch64.dmg` | M1 / M2 / M3 / M4 等芯片 |
| 🍎 **macOS**（Intel） | `Yuki_x.y.z_x64.dmg` | Intel 芯片 Mac |
| 🐧 **Linux**（x64） | `.AppImage` / `.deb` | AppImage 通用；`.deb` 适用于 Debian / Ubuntu 系 |

> [!IMPORTANT]
> 当前安装包**未经代码签名**，首次打开时系统可能会拦截，按下方说明放行即可。

<details>
<summary><b>🪟 Windows：出现「Windows 已保护你的电脑」</b></summary>

<br/>

在 SmartScreen 弹窗中点击 **「更多信息」→「仍要运行」**。

</details>

<details>
<summary><b>🍎 macOS：提示「已损坏，无法打开」或「无法验证开发者」</b></summary>

<br/>

1. 打开 DMG，将 **Yuki** 拖入「应用程序」文件夹。
2. 在终端执行以下命令移除隔离标记：

   ```bash
   xattr -cr /Applications/Yuki.app
   ```

3. 再次打开 Yuki。也可以在 Finder 中右键 Yuki →「打开」，或前往「系统设置 → 隐私与安全性」点击「仍要打开」。

</details>

<details>
<summary><b>🐧 Linux：运行 AppImage / 安装 deb</b></summary>

<br/>

```bash
chmod +x Yuki_*.AppImage
./Yuki_*.AppImage
```

```bash
sudo apt install ./Yuki_*.deb
```

</details>

## 🚀 快速上手

只需三步即可完成第一次下载：

```text
 ① 安装运行环境          ② 粘贴链接并解析          ③ 选择格式并下载
 设置 → 运行环境    →    输入框粘贴 → 解析    →    挑选清晰度 → 下载
```

1. **安装运行环境**：首次启动后打开 **设置 → 运行环境**，点击安装 **yt-dlp** 与 **FFmpeg**。Yuki 会从官方来源下载、校验并试运行，全程无需管理员权限。
   > macOS 上 FFmpeg 需自行安装（推荐 `brew install ffmpeg`），Yuki 会自动检测。
2. **粘贴链接**：复制视频链接，粘贴到顶部输入框（可在设置中开启「自动识别剪贴板链接」），点击 **解析**。
3. **选择并下载**：在解析面板中查看标题、封面和全部格式，选中想要的清晰度，点击 **下载此格式**。下载完成后可直接「打开文件」或「打开所在文件夹」。

## 📖 使用教程

### 1. 批量下载

在输入框中**每行粘贴一个链接**，Yuki 会提示「已识别 N 个链接」，解析后逐个加入下载队列。

### 2. 下载播放列表

粘贴播放列表链接后，解析面板会显示列表条目（预览最多读取前 500 条，不影响实际下载范围）：

- 点击 **下载整个列表** 下载全部条目；
- 在 **设置 → 下载 → 列表范围** 中填写区间，例如 `1-10,15` 表示第 1–10 条和第 15 条；
- **设置 → 下载 → 播放列表** 可选择「每次询问 / 按链接自动判断 / 只下载单个视频 / 下载整个列表」。

> [!TIP]
> YouTube 视频链接中带有 `list=` 参数时，若只想下载当前视频，将播放列表模式设为「只下载单个视频」即可。

### 3. 只下载音频

在解析面板选择 **仅音频** 轨道，或在 **设置 → 下载 → 默认清晰度** 中选择「仅音频」，并在「音频格式」中选择 FLAC / MP3 / M4A / Opus。音频转换需要 FFmpeg。

### 4. 字幕、封面与元数据

在 **设置 → 下载** 中可以开启：

| 选项 | 作用 |
| --- | --- |
| 写入元数据 | 将标题、作者等信息写入文件 |
| 嵌入封面 | 将视频封面嵌入文件（需要 FFmpeg） |
| 下载并嵌入字幕 | 字幕语言用逗号分隔，例如 `zh-Hans,en` |
| 包含自动生成字幕 | 同时下载平台自动生成的字幕 |
| 保存 info.json | 保存完整的视频元数据 |
| 下载归档文件 | 记录已下载的视频，避免重复下载 |

### 5. 配置 Cookie（登录后才能看的内容 / 高画质）

部分内容需要登录才能访问，例如会员视频、年龄限制视频，或 B 站 1080p60 / 4K / HDR 等画质。打开 **设置 → 网络与登录**，任选一种方式：

<details open>
<summary><b>方式 A：直接粘贴 Cookie（推荐，最稳定）</b></summary>

<br/>

1. 在浏览器中打开目标网站并登录，按 <kbd>F12</kbd> 打开开发者工具 → **网络（Network）**，刷新页面，选中任意一个发往该网站的请求。
2. 在 **请求头（Request Headers）** 中找到 `Cookie`，复制整行或它的值。
   > 注意不要复制响应头中的 `Set-Cookie`。
3. 在 Yuki 中选择 Cookie 所属网站：YouTube 选择 **YouTube**；其他平台选择 **其他网站（自定义）** 并填写实际来源域名，如 `https://www.bilibili.com`。
4. 粘贴并保存，然后重新解析。

Yuki 会在本地生成 Cookie 文件，保留其他域名已有的 Cookie，并自动停用浏览器读取。原始内容不会写入设置或任务参数，保存后输入框会清空。也支持直接粘贴完整的 Netscape `cookies.txt` 内容（自带域名，不受网站选项影响）。

</details>

<details>
<summary><b>方式 B：从浏览器读取</b></summary>

<br/>

在「从浏览器读取 Cookie」中选择已登录的浏览器。

> [!WARNING]
> Windows 上读取 Chrome / Edge 前请**完全退出浏览器（包括后台进程）**。若仍提示无法解密，请改用方式 A、方式 C 或 Firefox。读取失败时，错误面板中的 **「停用浏览器 Cookie 并重试」** 可以一键停用并重新解析公开内容。

</details>

<details>
<summary><b>方式 C：使用 cookies.txt 文件</b></summary>

<br/>

用浏览器扩展导出 Netscape 格式的 `cookies.txt`，在「Cookie 文件」中选择该文件。填写后只使用文件，不再读取浏览器。

</details>

> [!CAUTION]
> Cookie 等同于你的登录凭据，**请勿分享 Cookie 或包含 Cookie 值的截图**。

### 6. 代理与网络

在 **设置 → 网络与登录** 中：

- **代理**：例如 `http://127.0.0.1:7890` 或 `socks5://127.0.0.1:1080`，下载任务与依赖更新共用；使用 TUN 模式时无需填写。
- **限速**：例如 `2M` 表示每秒 2 MB，留空不限速。
- **分片并发数**：提高可加速支持分片的站点，过高可能被限速。
- **启用 JavaScript 运行时**：YouTube 需要 [Deno](https://deno.com) 或 [Node.js](https://nodejs.org) 解析签名，Yuki 会自动检测系统中的 `deno` / `node` / `bun` / `qjs`。

### 7. 管理下载任务

- **筛选**：按「全部 / 下载中 / 等待中 / 已暂停 / 已完成」查看任务。
- **暂停与继续**：暂停会保留临时分片，继续时断点续传。
- **详情**：点击任意任务打开侧边抽屉，查看实时日志、完整命令行（可一键复制，敏感信息已脱敏）、输出路径与退出码。
- **同时下载数**：在 **设置 → 下载** 中调整，超出的任务自动排队。

### 8. 外观与语言

**设置 → 外观** 中可切换深色（Matte Graphite）/ 浅色（Warm Stone）主题，以及简体中文 / English / 跟随系统。

## ❓ 常见问题

<details>
<summary><b>下载时报 <code>HTTP Error 403: Forbidden</code>（YouTube）</b></summary>

<br/>

网页解析成功不代表媒体请求一定可用。先检查 `youtube.com` 与 `*.googlevideo.com` 是否走同一个出口，尝试切换代理节点后重新解析。只有日志中另有登录或人机验证提示时，才需要配置 YouTube Cookie。

</details>

<details>
<summary><b>提示需要登录 / 只能看到低画质</b></summary>

<br/>

按 [配置 Cookie](#5-配置-cookie登录后才能看的内容--高画质) 设置有对应权限的账号 Cookie 后重新解析。例如 B 站 4K、HDR、1080p60 可能需要大会员。画质预设不会解锁网站未返回的格式。

</details>

<details>
<summary><b>提示缺少 FFmpeg / 无法合并音视频</b></summary>

<br/>

Windows / Linux：在 **设置 → 运行环境** 中一键安装 FFmpeg。macOS：执行 `brew install ffmpeg` 后重启 Yuki。

</details>

<details>
<summary><b>YouTube 提示缺少 JavaScript 运行时</b></summary>

<br/>

安装 [Deno](https://deno.com) 或 [Node.js](https://nodejs.org)，确认其在系统 `PATH` 中，并在设置中开启「启用 JavaScript 运行时」。

</details>

<details>
<summary><b>解析很慢 / 网站更新后无法解析</b></summary>

<br/>

在 **设置 → 运行环境** 中检查并更新 yt-dlp，网站改版后通常需要新版本适配。播放列表解析超过 120 秒时，界面会保留引擎日志，便于区分是网络等待还是列表处理。

</details>

<details>
<summary><b>想使用自己安装的 yt-dlp</b></summary>

<br/>

Yuki 按以下优先级查找 yt-dlp，不会向系统全局环境写入任何文件：

1. 环境变量 `YUKI_YTDLP_BIN` 指定的路径；
2. 应用托管目录（Windows：`%APPDATA%\dev.yuki.downloader\bin\yt-dlp.exe`；Linux / macOS：应用配置目录下的 `bin/yt-dlp`）；
3. 系统 `PATH` 中的 `yt-dlp`（winget、scoop、brew、apt、pipx 等安装）；
4. Python 模块回退：`PYTHON` → `python` → `python3` → `py`，以 `python -m yt_dlp` 启动。

</details>

---

## 🛠️ 开发与构建

### 环境要求

- **Node.js** 22+
- **Rust** stable 工具链
- 平台构建依赖：Linux 需 `libwebkit2gtk-4.1-dev`、`libayatana-appindicator3-dev`、`librsvg2-dev`、`patchelf` 等（详见 [`yuki-ci.yml`](.github/workflows/yuki-ci.yml)）

### 本地开发

```bash
npm install            # 安装前端与 Tauri CLI 依赖
npm run dev            # 启动完整桌面应用（连接真实 Rust 下载引擎）
npm run dev:frontend   # 仅启动前端预览：http://127.0.0.1:1420（使用模拟数据）
```

### 测试与校验

```bash
npm run check:release                                   # 版本一致性与许可证资源检查
npm run build:frontend                                  # 构建前端静态产物
cargo test --lib --manifest-path src-tauri/Cargo.toml   # Rust 单元与集成测试
```

### 打包

```bash
npm run build                      # 构建当前平台安装包
npm run build -- --bundles nsis    # 仅构建 Windows NSIS 安装程序
```

产物位于 `src-tauri/target/release/bundle/`（NSIS / MSI / DMG / AppImage / deb）。

### 命令行自检

编译后的 `yuki` 可执行文件内置无界面诊断参数（可用 `YUKI_CONFIG_DIR` 指定隔离的配置目录）：

```bash
yuki --check-updates     # 查询上游最新 yt-dlp 与 FFmpeg 版本（遵循代理配置）
yuki --install-ytdlp     # 下载、校验并安装托管 yt-dlp
yuki --install-ffmpeg    # 下载、校验并安装托管 FFmpeg
yuki --diagnose <url>    # 用真实参数执行一次诊断下载并输出完整命令行
```

### 开发脚本

```bash
npm run icons   # 从矢量绘制代码可复现地生成全套图标与 favicon

# 启动 dev:frontend 后，通过 Chrome DevTools Protocol 进行 UI 回归检查
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-native.png 1180 800 dark devscripts/verify-native-ui.js
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-settings.png 1180 800 dark devscripts/verify-settings.js
```

<details>
<summary><b>📂 项目结构</b></summary>

```text
.
├── index.html                  # 前端入口（首屏同步主题，消除白屏闪烁）
├── vite.config.js              # Vite + Tailwind CSS 4 构建配置
├── public/                     # 静态图标与首屏主题初始化脚本
├── src/                        # 原生 ES Module 前端
│   ├── main.js                 # 状态仓库、生命周期、Tauri 事件订阅与轮询兜底
│   ├── styles.css              # 设计系统与深 / 浅双主题令牌
│   ├── core/
│   │   ├── api.js              # Tauri IPC 桥接与浏览器预览模拟数据
│   │   ├── i18n.js             # 中英双语文案
│   │   └── util.js             # 格式化、链接提取、安全校验与 DOM 工具
│   └── ui/
│       ├── shell.js            # 应用外壳（自绘标题栏、侧栏、状态栏）
│       ├── preview.js          # 解析弹窗（格式列表与播放列表范围选择）
│       ├── tasklist.js         # 任务列表（按任务 ID 复用 DOM）
│       ├── drawer.js           # 任务详情抽屉（增量日志与脱敏命令行）
│       ├── settings.js         # 设置弹窗（草稿态编辑与运行环境托管）
│       ├── actions.js          # 全局事件委托与交互编排
│       ├── select.js           # 自定义下拉选择组件
│       ├── icons.js            # 内联 SVG 图标集
│       └── toasts.js           # 轻提示
├── src-tauri/                  # Tauri 2 / Rust 后端
│   ├── tauri.conf.json         # 窗口、CSP 与跨平台打包配置
│   ├── capabilities/           # Tauri 2 最小权限声明
│   ├── resources/              # 随包资源（抖音兼容插件）
│   └── src/
│       ├── lib.rs              # 命令注册、并发队列与 CLI 自检入口
│       ├── download.rs         # 参数构造、日志跟进、进度解析与进程树清理
│       ├── probe.rs            # 媒体解析、编码容错与格式优选
│       ├── cookies.rs          # 浏览器 Cookie 与多站点 cookies.txt 合并
│       ├── runtime.rs          # yt-dlp 与 JS 运行时探测
│       ├── deps.rs             # FFmpeg 检测与包管理器引导
│       ├── update.rs           # 在线更新、断点续传、SHA-256 校验与原子替换
│       └── settings.rs         # 配置与下载历史持久化
├── scripts/check-release.mjs   # 发布元数据与许可证检查
├── devscripts/                 # 图标生成与 CDP 界面回归脚本
└── docs/                       # 调研与修复记录
```

</details>

<details>
<summary><b>⚙️ 工程设计要点</b></summary>

- **结构化进度协议**：不依赖脆弱的终端输出正则，统一用 `--progress-template` 与 `--print` 输出机器可读字段，精确采集 DASH 音视频分离与字幕阶段的进度、字节数、速率与 ETA。
- **按需 DOM 更新**：外壳只挂载一次，任务卡片按 `task.id` 原地复用，进度通知节流合并，日志按序号增量追加，多任务高速下载时依然流畅。
- **Windows 编码与句柄稳定性**：子进程输出写入受控临时文件，由后台线程按字节偏移增量读取，规避管道句柄导致的 `OSError: [Errno 22]` 以及非 UTF-8 代码页（如 `cp936`）下的中文标题崩溃。
- **整棵进程树清理**：暂停或取消时，Windows 用 `taskkill /T /F`、Unix 用进程组信号一并终止 `ffmpeg` 子进程，避免孤儿进程锁定文件。

</details>

### CI 与发布

- **[`yuki-ci.yml`](.github/workflows/yuki-ci.yml)**：在 Windows、Ubuntu、macOS 上执行发布元数据检查、前端构建与 Rust 测试。
- **[`yuki-release.yml`](.github/workflows/yuki-release.yml)**：手动触发或推送 `yuki-v*` 标签时构建 Windows NSIS、Linux AppImage / deb、macOS（Apple Silicon 与 Intel）DMG；推送标签时会自动创建附带安装包的**草稿 Release**。手动触发时可勾选 `devtools` 生成带 Web Inspector 的诊断版本。

## 🤝 参与贡献

欢迎提交 Issue 与 Pull Request！提交前请确认：

1. `npm run check:release` 与 `cargo test --lib --manifest-path src-tauri/Cargo.toml` 通过；
2. 新增的界面文案同时提供中文与英文（见 [`src/core/i18n.js`](src/core/i18n.js)）；
3. 遵循 [`.editorconfig`](.editorconfig) 中的代码风格。

## ⚖️ 免责声明

Yuki 仅为 yt-dlp 与 FFmpeg 的图形界面，不提供、不托管任何媒体内容。请仅下载你有权保存的内容，并遵守所在地法律法规及目标网站的服务条款。

## 📄 许可证

Yuki 本体以 [**Unlicense**](LICENSE) 发布至公有领域。运行时按需下载的外部工具（官方独立版 yt-dlp 为 GPLv3+，托管 FFmpeg 为 LGPLv2.1+ 及其随附许可）以及内置字体与依赖各自保留原始许可证，详见 [**THIRD_PARTY_NOTICES.md**](THIRD_PARTY_NOTICES.md)。

## 🙏 致谢

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — 强大的视频下载引擎
- [FFmpeg](https://ffmpeg.org) 与 [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) — 音视频处理
- [Tauri](https://tauri.app) — 轻量跨平台桌面框架
- [Inter](https://rsms.me/inter/)、[Noto Sans SC](https://fonts.google.com/noto/specimen/Noto+Sans+SC)、[JetBrains Mono](https://www.jetbrains.com/lp/mono/) — 界面字体

<div align="center">

<sub>Made with ❄️ by Yuki contributors</sub>

</div>
