# MicuCodeLine Windows 用户手册

## 目录
- [简介](#简介)
- [系统要求](#系统要求)
- [安装步骤](#安装步骤)
- [配置说明](#配置说明)
- [使用方法](#使用方法)
- [常见问题](#常见问题)
- [故障排除](#故障排除)

## 简介

MicuCodeLine 是一个为 Claude Code 设计的高性能状态栏工具，专为 OpenClaudeCode 站定制。它提供：
- 实时余额显示
- 多主题支持
- 交互式 TUI 配置界面
- Git 状态、目录信息、上下文统计等多种信息段

## 系统要求

- Windows 10/11
- Claude Code CLI 工具
- 网络连接（用于 API 调用）

## 安装步骤

### 方法一：从源码编译（推荐）

1. **安装 Rust 工具链**

   访问 https://rustup.rs/ 下载并安装 Rust。

2. **克隆仓库**
   ```bash
   git clone https://github.com/zuoliangyu/MICUCODELINE.git
   cd MICUCODELINE
   ```

3. **编译项目**
   ```bash
   cargo build --release --features tui
   ```

4. **复制可执行文件**
   ```bash
   # 创建配置目录
   mkdir %USERPROFILE%\.claude\micucodeline

   # 复制可执行文件
   copy target\release\micucodeline.exe %USERPROFILE%\.claude\micucodeline\
   ```

### 方法二：从 Release 下载

1. 访问 https://github.com/zuoliangyu/MICUCODELINE/releases
2. 下载最新的 `micucodeline-windows-x64.zip`
3. 解压到 `%USERPROFILE%\.claude\micucodeline\` 目录

## 配置说明

### 1. 初始化配置文件

打开命令提示符或 PowerShell，运行：

```bash
%USERPROFILE%\.claude\micucodeline\micucodeline.exe --init
```

这将创建：
- `%USERPROFILE%\.claude\micucodeline\config.toml` - 主配置文件
- `%USERPROFILE%\.claude\micucodeline\themes\` - 主题目录

### 2. 配置 Claude Code

编辑 Claude Code 的配置文件 `%USERPROFILE%\.claude\settings.json`：

```json
{
  "statusLine": {
    "type": "command",
    "command": "%USERPROFILE%\\.claude\\micucodeline\\micucodeline.exe",
    "padding": 0
  }
}
```

**注意**：在 JSON 中，反斜杠需要转义为 `\\`。

### 3. 配置环境变量（必填）

在 `settings.json` 中添加 `env` 配置：

```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "你的认证令牌",
    "ANTHROPIC_BASE_URL": "https://www.openclaudecode.cn",
    "BALANCE_API_KEY": "你的系统访问令牌",
    "BALANCE_API_USER": "你的用户ID",
    "BALANCE_API_URL": "https://www.openclaudecode.cn/api/user/self"
  },
  "statusLine": {
    "type": "command",
    "command": "%USERPROFILE%\\.claude\\micucodeline\\micucodeline.exe",
    "padding": 0
  }
}
```

#### 获取 API 凭证

1. **系统访问令牌 (BALANCE_API_KEY)**
   - 登录 https://www.openclaudecode.cn
   - 进入：控制台 → 个人设置 → 安全设置 → 系统访问令牌
   - 点击"生成令牌"并复制

2. **用户 ID (BALANCE_API_USER)**
   - 在个人设置页面
   - 查看昵称下方的 ID 数字

3. **认证令牌 (ANTHROPIC_AUTH_TOKEN)**
   - 从 OpenClaudeCode 获取你的 API 密钥

### 4. 自定义配置（可选）

编辑 `%USERPROFILE%\.claude\micucodeline\config.toml` 来自定义：

```toml
# 主题设置
theme = "default"  # 可选: default, nord, dracula, gruvbox 等

# 启用的信息段
[segments]
model = true
directory = true
git = true
context = true
balance = true
branding = true

# 颜色配置
[colors]
primary = "#00D9FF"
secondary = "#FF6B9D"
success = "#00FF9F"
warning = "#FFB86C"
error = "#FF5555"
```

## 使用方法

### 基本命令

```bash
# 显示帮助信息
micucodeline --help

# 初始化配置
micucodeline --init

# 检查配置是否正确
micucodeline --check

# 打印当前配置
micucodeline --print

# 打开交互式配置界面
micucodeline --config

# 使用指定主题运行
micucodeline --theme nord
```

### 交互式配置界面

运行 `micucodeline --config` 进入 TUI 配置界面：

- **↑/↓** - 导航菜单
- **Enter** - 选择选项
- **Esc** - 返回/退出
- **Tab** - 切换面板

配置界面功能：
1. **主题选择** - 预览并切换不同主题
2. **信息段配置** - 启用/禁用各个信息段
3. **颜色自定义** - 调整颜色方案
4. **实时预览** - 查看配置效果

### 主题管理

#### 内置主题

- `default` - 默认主题
- `nord` - Nord 配色方案
- `dracula` - Dracula 配色方案
- `gruvbox` - Gruvbox 配色方案
- `monokai` - Monokai 配色方案
- `solarized-dark` - Solarized Dark
- `solarized-light` - Solarized Light

#### 自定义主题

1. 在 `%USERPROFILE%\.claude\micucodeline\themes\` 创建新的 `.toml` 文件
2. 定义颜色和样式
3. 在配置中引用主题名称

示例主题文件 `mytheme.toml`：

```toml
[colors]
primary = "#FF6B9D"
secondary = "#00D9FF"
success = "#00FF9F"
warning = "#FFB86C"
error = "#FF5555"
background = "#1E1E2E"
foreground = "#CDD6F4"

[styles]
bold = true
italic = false
```

## 常见问题

### Q: 状态栏不显示怎么办？

**A:** 检查以下几点：
1. 确认 `micucodeline.exe` 在正确的路径
2. 检查 `settings.json` 中的路径是否正确（注意反斜杠转义）
3. 运行 `micucodeline --check` 验证配置
4. 查看 Claude Code 的日志输出

### Q: 余额显示为 "N/A" 或不显示？

**A:** 可能的原因：
1. API 凭证配置错误 - 检查 `BALANCE_API_KEY` 和 `BALANCE_API_USER`
2. 网络连接问题 - 确认能访问 `https://www.openclaudecode.cn`
3. API URL 配置错误 - 确认 `BALANCE_API_URL` 正确

运行以下命令测试 API 连接：
```bash
curl -H "Authorization: Bearer YOUR_API_KEY" https://www.openclaudecode.cn/api/user/self
```

### Q: 如何更换主题？

**A:** 三种方法：
1. 临时切换：`micucodeline --theme nord`
2. 永久修改：编辑 `config.toml` 中的 `theme` 字段
3. 交互式选择：运行 `micucodeline --config` 进入配置界面

### Q: 如何禁用某些信息段？

**A:** 编辑 `config.toml`：

```toml
[segments]
model = true
directory = true
git = true
context = false      # 禁用上下文信息
balance = true
branding = false     # 禁用品牌标识
```

### Q: 状态栏更新不及时？

**A:** 余额信息有 5 分钟的缓存。如需立即刷新：
1. 删除缓存文件：`%USERPROFILE%\.claude\micucodeline\cache\balance_*.json`
2. 重启 Claude Code

### Q: 如何在多台电脑间同步配置？

**A:** 配置文件位于 `%USERPROFILE%\.claude\micucodeline\`，可以：
1. 使用云同步工具（OneDrive、Dropbox 等）
2. 使用 Git 管理配置文件
3. 手动复制 `config.toml` 和 `themes/` 目录

## 故障排除

### 问题：程序无法启动

**症状**：双击 `micucodeline.exe` 没有反应或闪退

**解决方案**：
1. 在命令行中运行查看错误信息：
   ```bash
   %USERPROFILE%\.claude\micucodeline\micucodeline.exe
   ```
2. 检查是否缺少 Visual C++ 运行库
3. 确认 Windows 版本兼容性

### 问题：配置文件损坏

**症状**：运行 `--check` 报错或程序行为异常

**解决方案**：
1. 备份当前配置：
   ```bash
   copy %USERPROFILE%\.claude\micucodeline\config.toml config.toml.backup
   ```
2. 重新初始化：
   ```bash
   micucodeline --init
   ```
3. 手动恢复自定义设置

### 问题：Git 信息不显示

**症状**：在 Git 仓库中状态栏不显示 Git 信息

**解决方案**：
1. 确认当前目录是 Git 仓库：`git status`
2. 检查 `config.toml` 中 `git = true`
3. 确认 Git 已正确安装并在 PATH 中

### 问题：性能问题

**症状**：状态栏更新缓慢或卡顿

**解决方案**：
1. 禁用不需要的信息段
2. 增加缓存时间（修改源码中的 `CACHE_FRESH_SECS`）
3. 检查网络连接速度
4. 使用更简单的主题

### 问题：中文显示乱码

**症状**：状态栏中文字符显示为方块或乱码

**解决方案**：
1. 确认终端支持 UTF-8 编码
2. 在 PowerShell 中运行：
   ```powershell
   [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
   ```
3. 设置系统区域为中文（控制面板 → 区域 → 管理 → 更改系统区域设置）

### 获取帮助

如果遇到其他问题：

1. **查看日志**：Claude Code 的日志可能包含错误信息
2. **GitHub Issues**：https://github.com/zuoliangyu/MICUCODELINE/issues
3. **官方网站**：https://www.openclaudecode.cn

## 更新日志

查看 `CHANGELOG.md` 了解版本更新历史。

## 许可证

MIT License - 详见 `LICENSE` 文件

## 致谢

- 原作者：Haleclipse (CCometixLine)
- OpenClaudeCode 团队
- 所有贡献者

---

**最后更新**：2026-01-17
**版本**：1.0.9
