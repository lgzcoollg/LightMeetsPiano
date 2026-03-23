use std::time::Duration;
#[cfg(target_os = "windows")]
use tokio::time::sleep;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use inputbot::KeybdKey;

#[cfg(target_os = "windows")]
pub async fn press_key(key: &str) {
    // 将窗口置于前台并激活
    windows::activate_window();

    // 解析按键并执行
    let key = parse_note(key);
    key.press();
    sleep(Duration::from_millis(100)).await;
    key.release();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub async fn press_key(_key: &str) {
    // 当前平台未实现按键模拟，直接跳过
}

#[cfg(target_os = "windows")]
pub fn parse_note(key: &str) -> KeybdKey {
    // 以Key为分隔符，分割字符串
    let parts: Vec<&str> = key.split("Key").collect();
    let key = parts[1];
    match key {
        "0" => KeybdKey::YKey,
        "1" => KeybdKey::UKey,
        "2" => KeybdKey::IKey,
        "3" => KeybdKey::OKey,
        "4" => KeybdKey::PKey,
        "5" => KeybdKey::HKey,
        "6" => KeybdKey::JKey,
        "7" => KeybdKey::KKey,
        "8" => KeybdKey::LKey,
        "9" => KeybdKey::SemicolonKey,
        "10" => KeybdKey::NKey,
        "11" => KeybdKey::MKey,
        "12" => KeybdKey::CommaKey,
        "13" => KeybdKey::PeriodKey,
        "14" => KeybdKey::SlashKey,
        _ => unreachable!(),
    }
}

// macOS: 使用 Enigo 进行键盘模拟
#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventTapLocation, CGKeyCode};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
#[cfg(target_os = "macos")]
use once_cell::sync::Lazy;
#[cfg(target_os = "macos")]
use std::sync::Mutex;
#[cfg(target_os = "macos")]
use std::sync::mpsc;
#[cfg(target_os = "macos")]
use dispatch::Queue;

#[cfg(target_os = "macos")]
pub async fn press_key(key: &str) {
    let key_for_parse = if let Some(pos) = key.rfind("Key") { &key[pos..] } else { key };
    if let Some(code) = parse_note(key_for_parse) {
        let _ = tokio::task::spawn_blocking(move || {
            if let Ok(src) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(down) = CGEvent::new_keyboard_event(src.clone(), code, true) {
                    down.post(CGEventTapLocation::HID);
                }
                std::thread::sleep(Duration::from_millis(100));
                if let Ok(up) = CGEvent::new_keyboard_event(src, code, false) {
                    up.post(CGEventTapLocation::HID);
                }
            }
        })
        .await;
    } else {
        println!("[macOS] 未识别的按键: {}", key);
    }
}

// macOS 同步阻塞版本：供命令层在阻塞线程中调用，避免异步上下文触发 worker 崩溃
#[cfg(target_os = "macos")]
pub fn press_key_blocking(key: &str) {
    let key_for_parse = if let Some(pos) = key.rfind("Key") { &key[pos..] } else { key };
    if let Some(code) = parse_note(key_for_parse) {
        static INPUT_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
        let _guard = INPUT_LOCK.lock().unwrap();
        if let Ok(src) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            if let Ok(down) = CGEvent::new_keyboard_event(src.clone(), code, true) {
                down.post(CGEventTapLocation::HID);
            }
            std::thread::sleep(Duration::from_millis(100));
            if let Ok(up) = CGEvent::new_keyboard_event(src, code, false) {
                up.post(CGEventTapLocation::HID);
            }
        }
    } else {
        println!("[macOS] 未识别的按键: {}", key);
    }
}

// macOS：单线程工作队列，顺序执行按键事件，避免频繁创建线程
#[cfg(target_os = "macos")]
static INPUT_TX: Lazy<mpsc::Sender<String>> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        let main_q = Queue::main();
        for key in rx {
            let key_for_parse = if let Some(pos) = key.rfind("Key") { &key[pos..] } else { &key };
            if let Some(code) = parse_note(key_for_parse) {
                main_q.exec_sync(move || {
                    if let Ok(src) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                        if let Ok(down) = CGEvent::new_keyboard_event(src.clone(), code, true) {
                            down.post(CGEventTapLocation::HID);
                        }
                        std::thread::sleep(Duration::from_millis(100));
                        if let Ok(up) = CGEvent::new_keyboard_event(src, code, false) {
                            up.post(CGEventTapLocation::HID);
                        }
                    }
                });
            } else {
                println!("[macOS] 未识别的按键: {}", key);
            }
        }
    });
    tx
});

// 入队接口：供命令层调用
#[cfg(target_os = "macos")]
pub fn enqueue_key(key: &str) {
    let _ = INPUT_TX.send(key.to_string());
}

#[cfg(target_os = "macos")]
pub fn parse_note(key: &str) -> Option<CGKeyCode> {
    if let Some(num) = key.strip_prefix("Key") {
        return match num {
            "0" => Some(16),
            "1" => Some(32),
            "2" => Some(34),
            "3" => Some(31),
            "4" => Some(35),
            "5" => Some(4),
            "6" => Some(38),
            "7" => Some(40),
            "8" => Some(37),
            "9" => Some(41),
            "10" => Some(45),
            "11" => Some(46),
            "12" => Some(43),
            "13" => Some(47),
            "14" => Some(44),
            _ => None,
        };
    }

    None
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;
    use inputbot::KeybdKey;

    #[test]
    fn test_parse_note() {
        assert_eq!(parse_note("Key0"), KeybdKey::YKey);
        assert_eq!(parse_note("Key1"), KeybdKey::UKey);
        assert_eq!(parse_note("Key2"), KeybdKey::IKey);
        assert_eq!(parse_note("Key3"), KeybdKey::OKey);
        assert_eq!(parse_note("Key4"), KeybdKey::PKey);
        assert_eq!(parse_note("Key5"), KeybdKey::HKey);
        assert_eq!(parse_note("Key6"), KeybdKey::JKey);
        assert_eq!(parse_note("Key7"), KeybdKey::KKey);
        assert_eq!(parse_note("Key8"), KeybdKey::LKey);
        assert_eq!(parse_note("Key9"), KeybdKey::SemicolonKey);
        assert_eq!(parse_note("Key10"), KeybdKey::NKey);
        assert_eq!(parse_note("Key11"), KeybdKey::MKey);
        assert_eq!(parse_note("Key12"), KeybdKey::CommaKey);
        assert_eq!(parse_note("Key13"), KeybdKey::PeriodKey);
        assert_eq!(parse_note("Key14"), KeybdKey::SlashKey);
    }

    #[tokio::test]
    async fn test_press_key() {
        press_key("Key1").await;
        press_key("Key2").await;
        press_key("Key3").await;
        press_key("Key4").await;
        press_key("Key5").await;
        press_key("Key6").await;
        press_key("Key7").await;
        press_key("Key8").await;
        press_key("Key9").await;
    }
}
