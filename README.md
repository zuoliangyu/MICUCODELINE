# MicuCodeLine

![Language:Rust](https://camo.githubusercontent.com/b858ce7ffb2054312ada07b2be7896f91eb95e0ca40f502793f23f96e0dd180d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c616e6775616765266d6573736167653d5275737426636f6c6f723d6f72616e6765267374796c653d666c61742d737175617265)
![License:MIT](https://camo.githubusercontent.com/c6a8d48e8b6ef330ef240499a811f77e629e4bdecc8f2327120137fb2406144d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c6963656e7365266d6573736167653d4d495426636f6c6f723d626c7565267374796c653d666c61742d737175617265)

MicuCodeLine 是 **MicuCode 专用版** Claude Code 状态栏工具，集成余额显示与主题/TUI 配置。

- 原作者仓库：https://github.com/Haleclipse/CCometixLine
- MicuCodeLine 官网：https://e-flowcode.cc

## 效果预览

![效果预览](https://github.com/zuoliangyu/MICUCODELINE/blob/master/assets/image.png)

## 功能特性

- **余额自动显示**：直接读取 Claude Code 已有配置，无需额外输入
- **三行 Powerline 布局**（v1.4.0）：上下文/花费/会话 → 路径/分支 → 已用/余额/MICU，固定霓虹渐变配色
- **t/s 吞吐显示**：Session 段渲染 `(+N,-N)  合计：X t/s`，从 transcript 累计 token + cost API 耗时实时计算
- Git / 目录 / 上下文 / 会话等常用 Segment
- 跨平台发布（macOS / Linux / Windows / ARM Linux 嵌入式设备）

> 状态栏的核心三段（**已用 / 余额 / MICU 品牌**）始终显示，保证一眼看到 MicuCode 账户状态。其余段（Model、Cwd、Git、Cost、Session、Directory 等）的开关、配色、图标、顺序、分隔符均可在 TUI 中编辑（`micucodeline --config`），按 `S` 保存到 `~/.claude/micucodeline/config.toml`，也可手动编辑该文件。主题保持单一内置（无 `--theme` 切换）。

## 安装

从 [Releases](https://github.com/zuoliangyu/MICUCODELINE/releases) 页面下载对应平台的二进制文件。

| 平台 | 文件名 | 说明 |
|------|--------|------|
| macOS x64 | `micucodeline-macos-x64.tar.gz` | Intel Mac |
| macOS ARM64 | `micucodeline-macos-arm64.tar.gz` | Apple Silicon (M1/M2/M3/M4) |
| Linux x64 | `micucodeline-linux-x64.tar.gz` | 标准 x86_64 Linux |
| Linux x64 静态 | `micucodeline-linux-x64-static.tar.gz` | musl 静态链接，兼容低版本 glibc |
| Linux ARM64 | `micucodeline-linux-arm64.tar.gz` | 树莓派 4/5、Jetson、RK3588 等 |
| Linux ARM64 静态 | `micucodeline-linux-arm64-static.tar.gz` | ARM64 musl 静态链接 |
| Linux ARMv7 | `micucodeline-linux-armv7.tar.gz` | 树莓派 2/3、32 位 ARM 嵌入式设备 |
| Windows x64 | `micucodeline-windows-x64.zip` | 64 位 Windows |

```bash
# macOS / Linux / ARM Linux
mkdir -p ~/.claude/micucodeline
wget https://github.com/zuoliangyu/MICUCODELINE/releases/latest/download/micucodeline-linux-arm64.tar.gz
tar -xzf micucodeline-linux-arm64.tar.gz
cp micucodeline ~/.claude/micucodeline/
chmod +x ~/.claude/micucodeline/micucodeline
```

Windows 直接下载 `.exe`，双击运行即可进入配置界面，程序会自动安装到 `%USERPROFILE%\.claude\micucodeline\` 目录。

## 配置 Claude Code 状态栏

在 `~/.claude/settings.json` 中加入：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline"
  }
}
```

Windows 示例：

```json
{
  "statusLine": {
    "command": "C:/Users/zuolan/.claude/micucodeline/micucodeline.exe",
    "padding": 0,
    "type": "command"
  }
}
```

## 余额显示说明

**无需任何额外配置。**

MicuCodeLine 自动读取 Claude Code 的 `~/.claude/settings.json` 中已有的环境变量：

| 变量 | 说明 |
|------|------|
| `ANTHROPIC_AUTH_TOKEN` 或 `ANTHROPIC_API_KEY` | 你的 API Key（MicuCode 控制台获取） |
| `ANTHROPIC_BASE_URL` | 中转站地址，如 `https://e-flowcode.cc` |

只要这两个变量已配置（Claude Code 正常工作的前提），余额就会自动显示，无需再做任何设置。

> **注意**：余额显示依赖 new-api 后台的「额度查询接口返回令牌额度而非用户额度」开关处于**关闭**状态（即返回用户账户余额而非 API Key 额度）。
> 管理员可在 `新API后台 → 设置 → 运营设置` 中找到该开关。

## 首次使用

直接双击可执行文件（或命令行运行），会自动弹出交互式主菜单，可在其中：

- 打开 TUI 配置面板（编辑非锁定段，按 `S` 保存到 `config.toml`）
- 检查配置

```bash
micucodeline --check       # 校验当前配置是否正确
micucodeline --print       # 输出当前配置内容
micucodeline --config      # 打开交互式 TUI 配置面板（可编辑保存）
micucodeline --init        # 在 ~/.claude/micucodeline/ 写入默认 config.toml
```
