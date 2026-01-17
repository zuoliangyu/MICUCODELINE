# @openclaudecode/micucodeline

MicuCodeLine 是 openclaudecode 站特供版 Claude Code 状态栏工具。

## 安装
```bash
npm install -g @openclaudecode/micucodeline
```

安装后默认路径：`~/.claude/micucodeline/micucodeline`

## 使用
```bash
micucodeline --help
micucodeline --version
```

## Claude Code 配置
在 `~/.claude/settings.json` 中设置：
```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline",
    "padding": 0
  }
}
```

## 余额配置
在 `settings.json` 的 `env` 中加入：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "xxx",
    "ANTHROPIC_BASE_URL": "xxx",
    "BALANCE_API_KEY": "YOUR_TOKEN",
    "BALANCE_API_USER": "12345"
  }
}
```

- 官网：https://www.openclaudecode.cn/
- 当前仓库：https://github.com/zuoliangyu/MICUCODELINE
- 原作者仓库：https://github.com/Haleclipse/CCometixLine
