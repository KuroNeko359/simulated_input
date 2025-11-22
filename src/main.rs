use clipboard::{
    ClipboardContext,
    ClipboardProvider
};
use enigo::{Enigo, Keyboard, Settings}; // Keyboard trait is not strictly needed here unless you use key presses directly
use std::{
    thread,
    time,
    io::{self, BufRead}, // 引入 I/O 模块来读取用户输入
};
use std::io::Write;

fn main() {
    // --- 初始化和设置 ---

    // 1. 初始化 Enigo
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // 2. 初始化剪贴板上下文
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(clipboard) => clipboard,
        Err(err) => {
            eprintln!("Clipboard error: {}", err);
            return;
        },
    };

    // 3. 打印启动信息
    println!("\n=============================================");
    println!("     ⌨️  剪贴板内容模拟输入工具已启动 ⌨️");
    println!("=============================================");
    println!("- 💡 **使用方法:**");
    println!("    1. 复制您想输入的内容到剪贴板。");
    println!("    2. 将光标定位到您想输入的位置 (例如：文本框)。");
    println!("    3. 按下 **Enter** 键 (回车键) 即可开始模拟输入。");
    println!("- ⏳ 每次按下 Enter 后，程序将等待 3 秒，让您有时间切换窗口。");
    println!("- ❌ 按 **Ctrl+C** 键 (或关闭窗口) 即可退出程序。");
    println!("=============================================\n");

    // --- 核心循环逻辑 ---

    // 创建一个标准输入读取器
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        // 1. 等待用户按 Enter
        print!("> 请按 Enter 键开始输入 (或 Ctrl+C 退出): ");
        // 必须刷新 stdout，否则 print! 的内容可能不会立即显示
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // EOF (例如 Ctrl+D)，虽然通常在终端里是 Ctrl+C 退出
                println!("\n接收到 EOF，程序退出。");
                break;
            }
            Ok(_) => {
                // 用户按下了 Enter 键
                // 2. 重新获取剪贴板内容 (确保获取的是最新的)
                let text_to_type = match ctx.get_contents() {
                    Ok(text) => text,
                    Err(err) => {
                        eprintln!("⚠️ 错误：无法获取剪贴板内容：{}", err);
                        continue; // 继续循环，等待下一次输入
                    },
                };

                if text_to_type.is_empty() {
                    println!("📢 警告：剪贴板内容为空，跳过本次输入。");
                    continue;
                }

                println!("⏳ 正在等待 3 秒... 请将光标切换到目标输入框。");
                thread::sleep(time::Duration::from_secs(3));

                // 3. 使用 Enigo 注入文本
                println!("⌨️ 模拟输入开始 ({} 个字符)...", text_to_type.len());
                // 使用 text() 方法直接注入文本
                if let Err(e) = enigo.text(&text_to_type) {
                    eprintln!("❌ 模拟输入失败：{:?}", e);
                } else {
                    println!("✅ 模拟输入完成！");
                }
            }
            Err(error) => {
                eprintln!("读取输入错误: {}", error);
                break;
            }
        }
    }
}