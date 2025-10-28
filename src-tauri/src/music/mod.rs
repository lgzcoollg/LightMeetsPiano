use std::time::Duration;
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
use enigo::{Enigo, KeyboardControllable, Key};

#[cfg(target_os = "macos")]
pub async fn press_key(key: &str) {
    let mut enigo = Enigo::new();
    let k = parse_note(key);
    enigo.key_down(k);
    // 使用阻塞睡眠，避免在 async 上下文中跨 await 持有非 Send 的 Enigo
    std::thread::sleep(Duration::from_millis(100));
    enigo.key_up(k);
}

#[cfg(target_os = "macos")]
pub fn parse_note(key: &str) -> Key {
    let parts: Vec<&str> = key.split("Key").collect();
    let key = parts[1];
    match key {
        "0" => Key::Layout('y'),
        "1" => Key::Layout('u'),
        "2" => Key::Layout('i'),
        "3" => Key::Layout('o'),
        "4" => Key::Layout('p'),
        "5" => Key::Layout('h'),
        "6" => Key::Layout('j'),
        "7" => Key::Layout('k'),
        "8" => Key::Layout('l'),
        "9" => Key::Layout(';'),
        "10" => Key::Layout('n'),
        "11" => Key::Layout('m'),
        "12" => Key::Layout(','),
        "13" => Key::Layout('.'),
        "14" => Key::Layout('/'),
        _ => Key::Layout(' '),
    }
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
