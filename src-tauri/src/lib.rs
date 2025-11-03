pub mod music;

#[tauri::command]
async fn press_key(key: String) {
    #[cfg(target_os = "macos")]
    {
        // 将按键事件入队，由单线程后台 worker 顺序执行
        music::enqueue_key(&key);
        // 直接返回，让命令非阻塞
        return;
    }

    #[cfg(not(target_os = "macos"))]
    {
        music::press_key(&key).await;
    }
}

// 返回当前前台应用信息：名称与 Bundle ID（仅 macOS 有效）
#[derive(serde::Serialize)]
struct FrontAppInfo {
    name: String,
    bundle_id: String,
}

#[tauri::command]
async fn front_app_info() -> Option<FrontAppInfo> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // 获取当前前台应用名称
        let name_output = Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to get name of first process whose frontmost is true")
            .output()
            .ok()?;
        if !name_output.status.success() {
            return None;
        }
        let name = String::from_utf8_lossy(&name_output.stdout).trim().to_string();

        // 获取当前前台应用的 Bundle ID（可能为空）
        let bid_output = Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to get bundle identifier of first process whose frontmost is true")
            .output()
            .ok()?;
        let bundle_id = if bid_output.status.success() {
            String::from_utf8_lossy(&bid_output.stdout).trim().to_string()
        } else {
            String::new()
        };

        Some(FrontAppInfo { name, bundle_id })
    }

    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

// 激活并切换到 Sky.app（或其不同名称/Bundle ID 变体）
#[tauri::command]
async fn focus_sky_app() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // 优先用 Bundle ID 激活（更稳定），失败再用应用名，最后尝试 open -a
        let bundle_scripts = [
            r#"tell application id \"com.tgc.sky\" to activate"#,
            r#"tell application id \"tgc.sky\" to activate"#,
        ];

        for script in bundle_scripts.iter() {
            if let Ok(out) = Command::new("osascript").arg("-e").arg(script).output() {
                if out.status.success() {
                    return Ok(());
                }
            }
        }

        let name_scripts = [
            r#"tell application \"Sky\" to activate"#,
            r#"tell application \"Children of the Light\" to activate"#,
            r#"tell application \"光遇\" to activate"#,
            r#"tell application \"光·遇\" to activate"#,
        ];

        for script in name_scripts.iter() {
            if let Ok(out) = Command::new("osascript").arg("-e").arg(script).output() {
                if out.status.success() {
                    return Ok(());
                }
            }
        }

        // 最后尝试用 open -a 启动并前置
        let fallback_names = ["Sky", "Children of the Light", "光遇", "光·遇"];
        for name in fallback_names.iter() {
            if let Ok(status) = Command::new("open").arg("-a").arg(name).status() {
                if status.success() {
                    return Ok(());
                }
            }
        }

        return Err("无法激活 Sky.app（名称或 Bundle ID 皆失败）".into());
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("仅在 macOS 支持自动切换到 Sky.app".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![press_key, front_app_info, focus_sky_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
