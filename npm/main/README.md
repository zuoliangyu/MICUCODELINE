# @zuolan/micucodeline

MicuCodeLine 是 MICU OpenClaudeCode 站特供版 Claude Code 状态栏工具，集成余额显示与主题配置。

## 安装
```bash
npm install -g @zuolan/micucodeline
```

安装后自动复制到：`~/.claude/micucodeline/micucodeline`

## Claude Code 配置

在 `~/.claude/settings.json` 中添加 `statusLine` 和 `env`：

```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "你的 API Key（sk-xxx）",
    "ANTHROPIC_BASE_URL": "https://www.openclaudecode.cn"
  },
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline",
    "padding": 0
  }
}
```

Windows 示例：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "你的 API Key（sk-xxx）",
    "ANTHROPIC_BASE_URL": "https://www.openclaudecode.cn"
  },
  "statusLine": {
    "command": "C:/Users/zuolan/.claude/micucodeline/micucodeline.exe",
    "padding": 0,
    "type": "command"
  }
}
```

配置好后，MicuCodeLine 会自动从 `settings.json` 的 `env` 中读取 API Key 和 Base URL，通过 `/api/user/self` 接口自动获取用户真实余额，**无需额外配置**。

## 使用
```bash
micucodeline --help        # 查看帮助
micucodeline --init        # 初始化配置与主题
micucodeline --config      # 打开配置面板
micucodeline --theme nord  # 指定主题运行
```

## 链接
- 官网：https://www.openclaudecode.cn/
- GitHub：https://github.com/zuoliangyu/MICUCODELINE
- 原作者：https://github.com/Haleclipse/CCometixLine
