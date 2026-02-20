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

配置好后，MicuCodeLine 会自动从 `settings.json` 的 `env` 中读取 API Key 和 Base URL，通过 `/api/user/self` 接口自动获取余额，**无需额外配置 JWT 或用户 ID**。

## 高级配置（可选）

如果你的 API Key 设置了无限额度（余额显示 ∞），可以在 `~/.claude/micucodeline/balance_config.json` 中配置 Access Token 来显示真实余额：

```json
{
  "api_key": "你的 API Key",
  "access_token": "你的 Access Token（从 new-api 个人中心获取）",
  "new_api_user_id": 12345,
  "exchange_rate": 7.3,
  "quota_per_unit": 500000.0
}
```

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
