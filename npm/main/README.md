# MicuCodeLine

[![npm version](https://img.shields.io/npm/v/@zuolan/micucodeline)](https://www.npmjs.com/package/@zuolan/micucodeline)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

MicuCodeLine 是 **MicuCode 专用版** Claude Code 状态栏工具，Rust 编写，支持余额显示、多主题、TUI 配置面板以及智能终端宽度换行。

> GitHub: https://github.com/zuoliangyu/MICUCODELINE

## 安装

```bash
npm install -g @zuolan/micucodeline
```

安装完成后会自动：
1. 将二进制文件复制到 `~/.claude/micucodeline/`
2. 运行 `micucodeline --init` 初始化配置

## 配置 Claude Code

在 `~/.claude/settings.json` 中添加：

```json
{
  "statusCommand": "micucodeline"
}
```

## 功能特性

- **余额显示**：自动读取 Claude Code 已有 Token，无需额外配置
- **多模型支持**：Opus 4.6 / Sonnet 4.6 / Haiku 4.5 / 1M context 模型 / 第三方模型（GLM、Kimi、Qwen 等）
- **智能换行**：自动检测终端宽度，超出时按 Segment 换行，不截断内容
- **多主题**：内置 Default / Gruvbox / Nord / Tokyo Night / Rose Pine 等主题
- **TUI 配置面板**：运行 `micucodeline --config` 交互式调整所有设置
- **Git / 目录 / 上下文窗口 / 会话** 等多种 Segment

## 命令

```bash
micucodeline --help      # 查看帮助
micucodeline --init      # 初始化配置文件
micucodeline --config    # 打开 TUI 配置面板
micucodeline --check     # 检查配置是否合法
micucodeline --print     # 打印当前配置
```

## 自定义模型显示

编辑 `~/.claude/micucodeline/models.toml` 添加自定义模型：

```toml
[[models]]
pattern = "my-custom-model"
display_name = "My Model"
context_limit = 128000
```

## 支持平台

| 平台 | 包名 |
|------|------|
| macOS x64 | `@zuolan/micucodeline-darwin-x64` |
| macOS arm64 | `@zuolan/micucodeline-darwin-arm64` |
| Linux x64 | `@zuolan/micucodeline-linux-x64` |
| Linux x64 (musl) | `@zuolan/micucodeline-linux-x64-musl` |
| Windows x64 | `@zuolan/micucodeline-win32-x64` |

## License

MIT
