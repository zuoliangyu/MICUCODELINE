# Changelog

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
