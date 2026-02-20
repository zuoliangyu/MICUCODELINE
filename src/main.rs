use micucodeline::cli::Cli;
use micucodeline::config::{Config, InputData};
use micucodeline::core::{StatusLineGenerator, collect_all_segments};
use std::io::{self, IsTerminal};

/// 自动将可执行文件复制到 ~/.claude/micucodeline/ 目录
fn auto_install() {
    // 获取当前可执行文件路径
    let current_exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(_) => return,
    };

    // 获取目标目录
    let target_dir = match dirs::home_dir() {
        Some(home) => home.join(".claude").join("micucodeline"),
        None => return,
    };

    // 获取目标文件路径
    let exe_name = if cfg!(windows) {
        "micucodeline.exe"
    } else {
        "micucodeline"
    };
    let target_path = target_dir.join(exe_name);

    // 如果当前已经在目标目录运行，跳过复制
    if current_exe.parent() == Some(target_dir.as_path()) {
        return;
    }

    // 创建目标目录（如果不存在）
    if std::fs::create_dir_all(&target_dir).is_err() {
        return;
    }

    // 检查是否需要复制（目标不存在或版本不同）
    let should_copy = if target_path.exists() {
        // 比较文件大小，如果不同则更新
        match (
            std::fs::metadata(&current_exe),
            std::fs::metadata(&target_path),
        ) {
            (Ok(src_meta), Ok(dst_meta)) => src_meta.len() != dst_meta.len(),
            _ => true,
        }
    } else {
        true
    };

    if should_copy && std::fs::copy(&current_exe, &target_path).is_ok() {
        // 在 Unix 系统上设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&target_path, std::fs::Permissions::from_mode(0o755));
        }
        eprintln!("✅ 已自动安装到: {}", target_path.display());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 自动安装到 ~/.claude/micucodeline/
    auto_install();

    let cli = Cli::parse_args();

    // Handle configuration commands
    if cli.init {
        use micucodeline::config::InitResult;
        match Config::init()? {
            InitResult::Created(path) => println!("Created config at {}", path.display()),
            InitResult::AlreadyExists(path) => {
                println!("Config already exists at {}", path.display())
            }
        }
        return Ok(());
    }

    if cli.print {
        let mut config = Config::load().unwrap_or_else(|_| Config::default());

        // Apply theme override if provided
        if let Some(theme) = cli.theme {
            config = micucodeline::ui::themes::ThemePresets::get_theme(&theme);
        }

        config.print()?;
        return Ok(());
    }

    if cli.check {
        let config = Config::load()?;
        config.check()?;
        println!("✓ Configuration valid");
        return Ok(());
    }

    if cli.config {
        #[cfg(feature = "tui")]
        {
            micucodeline::ui::run_configurator()?;
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("TUI feature is not enabled. Please install with --features tui");
            std::process::exit(1);
        }
        return Ok(());
    }

    if cli.update {
        #[cfg(feature = "self-update")]
        {
            println!("Update feature not implemented in new architecture yet");
        }
        #[cfg(not(feature = "self-update"))]
        {
            println!("Update check not available (self-update feature disabled)");
        }
        return Ok(());
    }

    // Handle Claude Code patcher
    if let Some(claude_path) = cli.patch {
        use micucodeline::utils::ClaudeCodePatcher;

        println!("🔧 Claude Code Context Warning Disabler");
        println!("Target file: {}", claude_path);

        // Create backup in same directory
        let backup_path = format!("{}.backup", claude_path);
        std::fs::copy(&claude_path, &backup_path)?;
        println!("📦 Created backup: {}", backup_path);

        // Load and patch
        let mut patcher = ClaudeCodePatcher::new(&claude_path)?;

        println!("\n🔄 Applying patches...");
        let results = patcher.apply_all_patches();
        patcher.save()?;

        ClaudeCodePatcher::print_summary(&results);
        println!("💡 To restore warnings, replace your cli.js with the backup file:");
        println!("   cp {} {}", backup_path, claude_path);

        return Ok(());
    }

    // Load configuration
    let mut config = Config::load().unwrap_or_else(|_| Config::default());

    // Apply theme override if provided
    if let Some(theme) = cli.theme {
        config = micucodeline::ui::themes::ThemePresets::get_theme(&theme);
    }

    // Check if stdin has data
    if io::stdin().is_terminal() {
        // Check if this is first-time run (no config exists)
        #[cfg(feature = "tui")]
        {
            use std::path::PathBuf;

            // Try to get config path (使用与 Config::get_config_path() 相同的路径)
            let config_path: Option<PathBuf> = dirs::home_dir()
                .map(|p| p.join(".claude").join("micucodeline").join("config.toml"));

            let is_first_run = config_path.as_ref().map(|p| !p.exists()).unwrap_or(false);

            if is_first_run {
                // First-time run: silently initialize config and continue to main menu
                let _ = Config::init();
            }
        }

        // No input data available, show main menu
        #[cfg(feature = "tui")]
        {
            use micucodeline::ui::{MainMenu, MenuResult};

            if let Some(result) = MainMenu::run()? {
                match result {
                    MenuResult::LaunchConfigurator => {
                        micucodeline::ui::run_configurator()?;
                    }
                    MenuResult::InitConfig | MenuResult::CheckConfig => {
                        // Handled internally by the menu
                    }
                    MenuResult::Exit => {}
                }
            }
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("No input data provided and TUI feature is not enabled.");
            eprintln!("Usage: echo '{{...}}' | micucodeline");
            eprintln!("   or: micucodeline --help");
        }
        return Ok(());
    }

    // Read Claude Code data from stdin
    let stdin = io::stdin();
    let input: InputData = serde_json::from_reader(stdin.lock())?;

    // Collect segment data
    let segments_data = collect_all_segments(&config, &input);

    // Render statusline
    let generator = StatusLineGenerator::new(config);
    let statusline = generator.generate(segments_data);

    println!("{}", statusline);

    Ok(())
}
