# Yuki · 雪迹

**Yuki（雪迹）** 是一个基于 **Tauri 2** 与 **Rust** 构建的轻量级跨平台桌面视频下载器，围绕 **yt-dlp** 与 **FFmpeg** 提供专注于「**先解析、再选择、后下载**」的原生桌面体验。

本项目为独立的桌面客户端仓库，不内置或修改 `yt-dlp` 源码库；前端采用 **Tailwind CSS 4 + 原生 ES Modules**（无重型前端 UI 框架与运行时开销），后端下载队列、进程监管与依赖托管均由 **Rust** 实现。

---

## 核心特性

| 能力 | 说明 |
| --- | --- |
| **下载前解析** | 粘贴链接后先调用 `yt-dlp` 获取真实标题、封面、时长、作者、播放量及完整清晰度列表（含 `format_id`、编码、帧率、HDR、预估体积），确认后再下载。 |
| **真实格式选择** | 解析面板展示真实轨道规格，优先推荐直连 HTTPS 流（规避虚高码率 m3u8 分片 403 循环重试）；选中视频流时自动匹配最佳音频轨道合并。 |
| **批量与播放列表** | 支持多行批量输入（每行一个链接），支持整张播放列表下载或通过区间语法（如 `1-10,15`）按需选取条目。 |
| **多站点 Cookie 管理** | 支持从本地浏览器读取 Cookie（Chrome / Edge / Firefox / Brave / Opera / Vivaldi / Safari 等）、导入 Netscape `cookies.txt`，或按站点（YouTube / Bilibili / 抖音 / 自定义来源域名）直接粘贴请求头，无需制作文件。导入时保留其他域名的 Cookie；浏览器读取失败时可停用并重试公开内容。 |
| **抖音兼容插件** | 内置独立的 `yt-dlp` 抖音提取器兼容插件（通过 `--plugin-dirs` 挂载），在应用内升级 `yt-dlp` 后依然保留适配能力。 |
| **队列、并发与断点续传** | 可配置最大并发下载数，超出任务自动排队；支持暂停（保留临时分片）与继续（`--continue`），取消任务时连同 `ffmpeg` 子进程树一并清理。 |
| **代理与传输控制** | 支持 HTTP / HTTPS / SOCKS5 代理（下载任务与依赖更新共用）、速度限制、分片并发数及重试次数配置。 |
| **实时日志与可读诊断** | 点击任意任务即可在侧边抽屉查看增量实时日志、自动脱敏后的完整命令行参数与输出路径，并可将常见错误（未登录、缺少 JS 运行时、缺少 FFmpeg、403、超时等）转化为可操作的修复指引。 |
| **运行时托管与校验** | 安装包仅包含客户端本体；首次使用可在「设置 → 运行环境」一键下载官方独立版 `yt-dlp` 与 `FFmpeg`，全程执行 SHA-256 校验、试运行与失败回滚，无需预装 Python，无需管理员权限。 |
| **原生双主题与双语** | 提供 Matte Graphite（深色）与 Warm Stone（浅色）两套不透明桌面材质主题，支持简体中文 / English 即时切换，内置本地自托管字体（Inter / Noto Sans SC / JetBrains Mono）。 |

---

## 直接粘贴 Cookie

1. 在浏览器打开目标网站，完成必要的登录或验证；按 F12 → 网络，选择发往该网站的请求。
2. 复制请求头中的 `Cookie` 整行或完整值，不要复制响应头 `Set-Cookie` 或整个请求。
3. 打开「设置 → 网络与登录」。YouTube 选择 YouTube；其他平台选择「其他网站（自定义）」，填写实际来源域名或网址。自定义来源保留提供的主机名，不推测主域名，Cookie 同时适用于它的子域名。
4. 粘贴并保存，然后重新解析。应用生成本地 Cookie 文件，保留其他域名的 Cookie，并关闭浏览器读取。原始内容不写入设置或任务参数，保存后输入框清空。完整 Netscape 内容自带域名，不受网站选项影响。

浏览器数据库读取失败不代表视频本身无法解析。错误面板的「停用浏览器 Cookie 并重试」会保存停用设置，后续解析和下载均不再读取浏览器；需要登录或验证的内容仍需有效 Cookie。不要分享 Cookie 或含 Cookie 值的截图。

YouTube 网页解析成功不代表媒体请求一定可用。若下载日志显示 `HTTP Error 403: Forbidden`，先检查 `youtube.com` 与 `*.googlevideo.com` 的出口是否一致，并尝试切换出口后重新解析；TUN 模式不必重复填写代理。只有日志另有登录或人机验证提示时，再处理对应 YouTube Cookie。下载的标准输出和错误输出共享日志文件的写入位置，避免进度覆盖错误；日志文件无法创建时直接报告错误，不启动无法采集输出的任务。

---

## 运行时解析机制

**播放列表预览**：解析阶段只读取列表条目，不逐个请求全部视频的格式；YouTube 视频链接附带 `list=` 时也采用此方式。预览最多读取前 500 条，点击条目后再解析该视频的完整格式；此限制不改变实际下载的列表范围。「只下载单个视频」仍优先解析当前视频。若解析总耗时超过 120 秒，界面保留引擎日志，便于区分网络等待与列表处理。

Yuki 不向系统全局环境写入文件，按以下优先级自动定位 `yt-dlp`：

1. **显式环境变量**：`YUKI_YTDLP_BIN`（适用于自定义二进制路径或外部 sidecar）。
2. **应用托管目录**：由 Yuki 在应用数据目录内下载并完成 SHA-256 校验的官方独立版 `yt-dlp`（无需系统安装 Python，应用内更新后立即生效）：
   - Windows：`%APPDATA%\dev.yuki.downloader\bin\yt-dlp.exe`
   - Linux / macOS：系统应用配置目录下的 `bin/yt-dlp`
3. **系统 `PATH`**：环境中已安装的 `yt-dlp` 可执行文件（如通过 `winget`、`scoop`、`brew`、`apt`、`pipx` 安装）。
4. **Python 模块回退**：自动探测可用的 Python 解释器（`PYTHON` 环境变量 → `python` → `python3` → `py`），若已通过 `pip install yt-dlp` 安装则以 `python -m yt_dlp` 启动。

**辅助运行时：**
- **FFmpeg / ffprobe**：用于音视频流合并、封装转换、提取无损/有损音频（FLAC / MP3 / M4A / Opus）及嵌入封面与字幕。Windows / Linux 支持在应用内一键托管下载 BtbN LGPL shared 构建（保留完整动态库与许可证）；macOS 支持检测系统 / Homebrew 安装并提供引导。
- **JavaScript 运行时**：用于 YouTube 签名挑战解析，自动检测系统中的 `deno`、`node`、`bun` 或 `qjs`。

---

## 开发与构建

### 环境要求

- **Node.js** 22+（前端构建，同时可作为 `yt-dlp` 的本地 JS 运行时）
- **Rust** stable 工具链（Tauri 2 桌面端与核心下载引擎）
- 平台桌面构建依赖（Linux 需 `libwebkit2gtk-4.1-dev`、`libayatana-appindicator3-dev`、`librsvg2-dev`、`patchelf` 等，详见 [`.github/workflows/yuki-ci.yml`](.github/workflows/yuki-ci.yml)）

### 本地开发

在仓库根目录直接执行：

```powershell
# 安装前端与 Tauri CLI 依赖
npm install

# 启动完整桌面应用（推荐，连接真实 Rust 下载引擎）
npm run dev

# 仅启动前端界面预览（浏览器访问 http://127.0.0.1:1420，使用内置模拟数据）
npm run dev:frontend
```

### 测试与校验

```powershell
# 校验包版本一致性（package.json / Cargo.toml / tauri.conf.json）及打包许可证资源
npm run check:release

# 构建前端静态产物
npm run build:frontend

# 运行 Rust 核心库单元与集成测试（参数构造、进度解析、Cookie 隔离、编码容错等）
cargo test --lib --manifest-path src-tauri/Cargo.toml
```

### 打包安装包

```powershell
# 构建当前平台安装包
npm run build

# 仅构建 Windows NSIS 安装程序
npm run build -- --bundles nsis
```

构建产物位于 `src-tauri/target/release/`（可执行文件）及 `src-tauri/target/release/bundle/`（NSIS / MSI / DMG / AppImage / deb 安装包）。

---

## 命令行诊断与开发脚本

编译后的 `yuki` 可执行文件内置无界面自检参数，便于排查网络代理、依赖更新与参数构造问题（可通过 `YUKI_CONFIG_DIR` 指定隔离配置目录）：

```powershell
yuki --check-updates        # 查询上游最新 yt-dlp 与 FFmpeg 版本（遵循代理配置）
yuki --install-ytdlp        # 下载、校验并安装托管 yt-dlp
yuki --install-ffmpeg       # 下载、校验并安装托管 FFmpeg
yuki --diagnose <url>       # 使用真实参数执行一次诊断下载并输出完整命令行
```

**图标生成与 UI 回归检查：**

```powershell
# 从矢量绘制代码可复现地生成全套应用图标与 favicon
npm run icons

# 启动 npm run dev:frontend 后，通过 Chrome DevTools Protocol 执行自动化 UI 几何与交互回归校验
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-native.png 1180 800 dark devscripts/verify-native-ui.js
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-settings.png 1180 800 dark devscripts/verify-settings.js
```

---

## 项目结构

```text
.
├── index.html                  # 前端入口（含首屏同步主题预设，消除白屏闪烁）
├── package.json                # 前端依赖与开发/构建脚本
├── vite.config.js              # Vite + Tailwind CSS 4 构建配置
├── public/                     # 静态图标与首屏主题初始化脚本
├── src/                        # 原生 ES Module 前端源码
│   ├── main.js                 # 状态仓库、生命周期、Tauri 事件订阅与轮询兜底
│   ├── styles.css              # 设计系统与深/浅双主题令牌
│   ├── core/
│   │   ├── api.js              # Tauri IPC 桥接与浏览器预览模式模拟数据
│   │   ├── i18n.js             # 中英双语文案字典
│   │   └── util.js             # 格式化、链接提取、安全校验与 DOM 工具
│   └── ui/
│       ├── shell.js            # 应用外壳（自绘标题栏、侧栏、状态栏，按需补丁更新）
│       ├── preview.js          # 链接解析弹窗（真实格式列表与播放列表范围选择）
│       ├── tasklist.js         # 下载任务列表（按任务 ID 复用 DOM，零闪烁）
│       ├── drawer.js           # 任务详情抽屉（增量日志流与脱敏命令行）
│       ├── settings.js         # 设置弹窗（草稿态编辑与运行环境托管更新）
│       ├── actions.js          # 全局事件委托与用户交互编排
│       ├── icons.js            # 内联单色 SVG 图标集
│       └── toasts.js           # 轻提示组件
├── src-tauri/                  # Tauri 2 / Rust 桌面后端
│   ├── Cargo.toml              # Rust 依赖清单
│   ├── tauri.conf.json         # 窗口、CSP 安全策略与跨平台打包配置
│   ├── capabilities/           # Tauri 2 最小权限声明（main.json）
│   ├── icons/                  # 跨平台应用图标资产
│   ├── resources/              # 随包分发的资源（内置抖音兼容插件等）
│   └── src/
│       ├── main.rs             # 二进制入口
│       ├── lib.rs              # Tauri 命令注册、并发下载队列编排与 CLI 自检入口
│       ├── download.rs         # 参数构造、临时文件日志跟进、结构化进度解析与进程树清理
│       ├── probe.rs            # 媒体解析（--dump-single-json）、编码容错与格式优选排序
│       ├── cookies.rs          # 浏览器 Cookie 参数与多站点 cookies.txt 合并校验
│       ├── runtime.rs          # yt-dlp 与 JS 运行时探测及缓存
│       ├── deps.rs             # FFmpeg 状态检测与系统包管理器引导
│       ├── update.rs           # yt-dlp / FFmpeg 在线版本检索、断点续传、SHA-256 校验与原子替换
│       └── settings.rs         # 配置与下载历史持久化
├── scripts/
│   └── check-release.mjs       # 发布元数据、版本号一致性与许可证打包检查
├── devscripts/                 # 图标生成脚本（make-icons.ps1）与 CDP 界面回归测试脚本
├── docs/                       # 架构调研与历史版本演进记录
├── licenses/                   # 自托管字体的 SIL Open Font License 文本
├── THIRD_PARTY_NOTICES.md      # 第三方组件与外部下载工具的许可证说明
└── LICENSE                     # Unlicense 许可证
```

---

## 工程设计要点

- **结构化进度协议**：放弃对人类可读终端输出的脆弱正则匹配，统一通过 `--progress-template` 与 `--print` 输出机器可读字段，精确采集多流（DASH 音视频分离）与字幕阶段的进度、已下载字节、实时速率与 ETA。
- **按需 DOM 更新**：界面外壳仅挂载一次，任务卡片按 `task.id` 原地复用 DOM 节点，进度通知按节流窗口合并推送，日志按序号增量追加，多任务高速下载时界面依然保持流畅响应。
- **Windows 编码与句柄稳定性**：子进程输出写入受控临时文件并由后台线程按字节偏移增量跟进，彻底规避 Windows 管道句柄导致的 `OSError: [Errno 22] Invalid argument` 以及 PyInstaller 冻结版二进制在非 UTF-8 系统代码页（如 `cp936`）下的中文标题崩溃问题。
- **整棵进程树清理**：暂停或取消任务时，Windows 下通过 `taskkill /T /F`、Unix 下通过进程组信号一并终止派生的 `ffmpeg` 子进程，防止孤立进程锁死输出文件。

---

## CI、发布与许可证

- **CI 工作流**（[`.github/workflows/yuki-ci.yml`](.github/workflows/yuki-ci.yml)）：在 Windows、Ubuntu 与 macOS 上自动执行发布元数据检查、前端构建与 Rust 单元测试。
- **打包工作流**（[`.github/workflows/yuki-release.yml`](.github/workflows/yuki-release.yml)）：手动触发或推送 `yuki-v*` 标签时构建 Windows NSIS、Linux AppImage/deb、macOS（Apple Silicon & Intel）DMG 产物并上传至 Actions Artifacts。默认产物未代码签名，公开发布前请配置自有证书与 macOS Notarization。
- **许可证**：Yuki 本体采用 [**Unlicense**](LICENSE) 发布至公有领域。应用运行期按需下载的外部工具（如官方独立版 `yt-dlp` 为 GPLv3+，托管 `FFmpeg` 为对应构建的 LGPLv2.1+ / 随附许可）及应用内置字体与依赖各自保留其原始许可证，详见 [**THIRD_PARTY_NOTICES.md**](THIRD_PARTY_NOTICES.md)。
