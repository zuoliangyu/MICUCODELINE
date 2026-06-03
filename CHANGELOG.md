# Changelog

## [1.5.0] - 2026-06-03

> 修复 1.4.0 重写后余额获取失效；模型名直接显示，不再经 models.toml 转录。

### Fixed

- **余额获取恢复**：1.4.0 重写为 EFlowCodeLine 架构时丢掉了「用环境 URL + API Key 直接查余额」的代码路径，导致只配了 API Key 的用户余额段位静默消失。现已恢复，并改为两段式：
  - 先尝试用 API Key 直连 `/api/user/self`（部分中转站支持，返回真实分组与额度）
  - 失败再回退标准 billing 接口（`/v1/dashboard/billing/subscription` + `usage`）
  - 两条路径都只使用 `~/.claude/settings.json` env（或系统环境变量）里的 `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN`/`ANTHROPIC_API_KEY`，无需任何额外配置
  - 修复了 micuapi 这类「`/api/user/self` 需要 session token、API Key 仅能访问 billing 接口」的中转站余额不显示的问题

### Changed

- **模型段直接显示获取到的名称**：不再把 model id 经 `models.toml` 的 pattern 转录成展示名。此前未匹配到的模型会落到通用兜底（如 `claude-opus-4-8[1m]` → 「Claude 1M」），显示很怪。现在直接采用 Claude Code 传入的 `display_name`（如「Opus 4.8 (1M)」）。`models.toml` 仍用于上下文窗口段的 token 上限匹配。

### Removed

- `ModelConfig::get_display_name`：模型名转录逻辑已废弃，随之移除

## [1.4.1] - 2026-05-28

> 部分开放配置：核心三段保持锁定，其余允许用户自定义。

### Added

- 重新支持 `~/.claude/micucodeline/config.toml`：TUI 中按 `S` 保存，或手动编辑文件
- `Config::load()` 自动调用 `enforce_locks()`：保证 `Balance` / `Used` / `Branding` 始终存在且 `enabled = true`，即使 config.toml 被改坏也会回退
- `Config::check()` 新增锁定段校验：缺失或被禁用时返回明确错误信息

### Changed

- TUI 配置面板从只读改回可写：保留段顺序、配色、图标、bold、分隔符的编辑能力
- `[W]` / `[Ctrl+S]` 与 `[S]` 行为统一为「保存到 config.toml」，移除 v1.4.0 的「保存主题」死路径
- README 与帮助文本同步：去掉「config.toml 已废弃」的旧描述

### Locked (intentionally)

- `Balance` / `Used` / `Branding` 三段不可禁用、不可移除（颜色 / 图标 / 位置仍可调）
- 主题仍为单一内置 Powerline 渐变（无 `--theme` 切换）

## [1.4.0] - 2026-05-23

> 大版本重写：基于 EFlowCodeLine 1.7.0 架构整体重构，品牌切换为 MICU。

### Added

- 三行 Powerline 状态栏布局（行 1：模型/上下文/花费/会话；行 2：cwd/dir/git；行 3：已用/余额/MICU）
- 新增 `Cwd` 段位：显示完整工作目录，自动 `$HOME → ~` 缩写，超过 5 段时中间用 `...` 折叠
- 新增 `Used` 段位：从 new-api `/api/user/self` 计算累计已用额度，固定 USD 显示
- `Session` 段位新增 **t/s 吞吐显示**：解析 transcript 内 assistant 消息累计 input+output tokens，除以 cost 数据中的 API 总耗时，渲染为 `(+N,-N)  合计：X t/s` 格式（参考 MicuSubCodeLine）
- `Branding` 段默认文本 `MICU`，固定显示
- 模型识别补齐 Opus 4.7 / Sonnet 4.7 / Haiku 4.7（含 `[1m]` 1M 上下文变体）
- 新增 ARM Linux 构建：`aarch64-unknown-linux-gnu`、`aarch64-unknown-linux-musl`、`armv7-unknown-linux-gnueabihf`
- 新增 3 个 NPM 平台包：`@zuolan/micucodeline-linux-arm64`、`@zuolan/micucodeline-linux-arm64-musl`、`@zuolan/micucodeline-linux-armv7`
- CI 使用 `cross` 工具实现 ARM 交叉编译
- NPM 安装脚本自动检测 ARM 架构并选择正确的二进制文件
- 可配置品牌静态文本

### Changed

- **状态栏锁定为单一 Powerline 主题**：内置霓虹渐变配色（深蓝→紫→粉→金），不再支持主题切换
- 段位顺序硬编码：`Cwd` 段前自动换到第 2 行，`Used` 段前自动换到第 3 行
- `Balance` 段位重写：拆出已用额度，仅输出 `余额:$X`，无限额度时显示 `余额:∞`
- `Used` 与 `Balance` 共享一次 HTTP 请求（缓存于内存 + 磁盘）

### Removed

- **Breaking**：删除 8 个旧主题文件（cometix/minimal/gruvbox/nord/powerline-dark/powerline-light/powerline-rose-pine/powerline-tokyo-night），仅保留单一硬编码主题
- **Breaking**：删除 `--theme` 命令行参数
- **Breaking**：不再读取 `~/.claude/micucodeline/config.toml`，所有主题/段位定义硬编码在二进制中
- **Breaking**：删除 `Group` 段位（new-api billing 接口不返回 group 信息）
- **Breaking**：`BalanceConfig` 删除 `exchange_rate` 与旧 `user_id` 字段（金额固定 USD，不再做汇率换算）
- TUI 配置面板的"保存主题"按钮改为只读提示，不再写盘

### Fixed

- `generate_wrapped` 单行短路路径未尊重段间强制换行的 bug

## [1.3.3] - 2026-05-16

### Fixed

- 修正 1M 模型 `claude-opus-4-6[1m]` 格式的模式匹配

## [1.3.2] - 2026-05-16

### Fixed

- 修复 clippy `collapsible_if` 警告
- 修正 1M 和 4.x 系列模型显示名称

## [1.2.0] - 2026-03-14

### Added

- 终端宽度自动行换行

## [1.1.x] - 2026-02

### Changed

- 多次小版本迭代，配置自动检测、文档更新、clippy 修复
