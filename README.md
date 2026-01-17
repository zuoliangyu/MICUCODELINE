# MicuCodeLine

![Language:Rust](https://camo.githubusercontent.com/b858ce7ffb2054312ada07b2be7896f91eb95e0ca40f502793f23f96e0dd180d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c616e6775616765266d6573736167653d5275737426636f6c6f723d6f72616e6765267374796c653d666c61742d737175617265)
![License:MIT](https://camo.githubusercontent.com/c6a8d48e8b6ef330ef240499a811f77e629e4bdecc8f2327120137fb2406144d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c6963656e7365266d6573736167653d4d495426636f6c6f723d626c7565267374796c653d666c61742d737175617265)

MicuCodeLine 是 **MICU OpenClaudeCode 站特供版** Claude Code 状态栏工具，集成余额显示与主题/TUI 配置。
- MICU 原作者仓库：https://github.com/Haleclipse/CCometixLine
- OpenClaudeCode 官网：https://www.openclaudecode.cn

## 效果预览

![效果预览](https://github.com/zuoliangyu/MICUCODELINE/blob/master/assets/image.png)

## 功能特性
- 余额显示：对接 OpenClaudeCode new-api `/api/user/self`
- 多主题/交互式 TUI 配置
- Git/目录/上下文/会话等常用 Segment
- 跨平台发布（macOS/Linux/Windows）

## 安装

### npm 安装（推荐）
```bash
npm install -g @zuolan/micucodeline
```

安装后默认路径：`~/.claude/micucodeline/micucodeline`

### 手动安装（Release）
```bash
# 以 macOS x64 为例
mkdir -p ~/.claude/micucodeline
wget https://github.com/zuoliangyu/MICUCODELINE/releases/latest/download/micucodeline-macos-x64.tar.gz

tar -xzf micucodeline-macos-x64.tar.gz
cp micucodeline ~/.claude/micucodeline/
chmod +x ~/.claude/micucodeline/micucodeline
```

## Claude Code 配置
在 `~/.claude/settings.json` 中加入：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline",
    "padding": 0
  }
}
```
或者
```json
  "model": "opus",
  "statusLine": {
    "command": "%USERPROFILE%\\.claude\\micucodeline\\micucodeline.exe",
    "padding": 0,
    "type": "command"
  }
```

> 说明：`micucodeline --init` 只会生成本工具的 `config.toml` 和主题文件，**不会**自动修改 `settings.json`。

## 余额配置（重要）

### 方式一：使用配置工具（推荐）

**无论是通过 npm 安装还是手动安装，安装完成后都必须先运行配置工具：**

```bash
# Linux/macOS
~/.claude/micucodeline/micucodeline

# Windows
C:\Users\你的用户名\.claude\micucodeline\micucodeline.exe
```

运行后会进入交互式配置界面，按照提示输入：
1. **API Token（系统访问令牌）**：在 OpenClaudeCode 控制台 → 左上角个人设置 → 安全设置 → 系统访问令牌 → 生成令牌
2. **用户 ID**：在个人设置页面，用户名称下方显示的 ID

![配置参考图](https://github.com/zuoliangyu/MICUCODELINE/blob/master/assets/image2.png)

### 方式二：手动配置

在 `~/.claude/settings.json` 的 `env` 中添加以下变量：

- `BALANCE_API_KEY`：系统访问令牌
- `BALANCE_API_USER`：用户 ID（昵称下方）
- `BALANCE_API_URL`：可选，默认 `https://www.openclaudecode.cn/api/user/self`

示例：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "xxx",
    "ANTHROPIC_BASE_URL": "xxx",
    "BALANCE_API_KEY": "YOUR_TOKEN",
    "BALANCE_API_USER": "12345",
    "BALANCE_API_URL": "https://www.openclaudecode.cn/api/user/self"
  },
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline",
    "padding": 0
  }
}
```

## 使用方式
```bash
micucodeline --init        # 初始化配置与主题目录
micucodeline --check       # 校验当前配置是否正确
micucodeline --print       # 输出当前配置内容
micucodeline --config      # 打开交互式 TUI 配置面板
micucodeline --theme nord  # 临时指定主题运行
```

## 声明
当前项目部分代码以及 review 由 Codex 完成。
