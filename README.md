<div align="center">
    <img src="src-tauri/icons/icon.png" width="250" alt="logo">
</div>
<div align="center">

# LightMeetsPiano

</div>
轻量级光遇自动弹琴脚本，纯按键模拟，使用 Tauri + Vue3 + TypeScript + Rust 开发。

## 使用

支持直接导入 Sky Studio 导出的 TXT 谱，直接将其导入并将屏幕焦点切换到游戏窗口即可开始演奏。

## 下载

前往 [Releases](https://github.com/StillMisty/LightMeetsPiano/releases) 下载最新版本

- windows 用户可下载 exe 或 msi 安装包，可直接点击授权运行
- Linux 用户可下载 AppImage 或 deb 包安装，运行请使用 sudo 权限运行
- Arch 用户可通过 AUR 直接安装

```bash
paru -S lightmeetspiano-bin
```

## 开发

```bash
git clone https://github.com/StillMisty/LightMeetsPiano
cd LightMeetsPiano
# 安装依赖
pnpm i
# 启动
pnpm tauri dev
# 打包
pnpm tauri build
```

## macOS 构建与打包

如果你在 macOS 上构建，本项目已配置好打包为 `.app` 与 `.dmg`。

- 前置条件：
  - 安装 Xcode 命令行工具：`xcode-select --install`
  - 安装 Rust 工具链：`curl https://sh.rustup.rs -sSf | sh`，并确保 `cargo` 在 `PATH`
  - 安装 Node.js 与 `pnpm`

- 构建命令：
  - 生产打包：`pnpm tauri build`

- 构建产物位置（build 到哪去了）：
  - `.app`：`src-tauri/target/release/bundle/macos/lightmeetspiano.app`
  - `.dmg`：`src-tauri/target/release/bundle/macos/*.dmg`

- 运行与安装：
  - 直接运行应用：`open src-tauri/target/release/bundle/macos/lightmeetspiano.app`
  - 打开安装镜像：`open src-tauri/target/release/bundle/macos/*.dmg`，在弹出的卷宗窗口里将 `lightmeetspiano.app` 拖拽到 `Applications`

- Gatekeeper 提示（未签名应用）：
  - 若提示“无法验证开发者”，右键应用选择“打开”，或执行：`xattr -dr com.apple.quarantine /Applications/lightmeetspiano.app`
  - 如需正式分发，请配置 Apple Developer ID 证书并进行签名与公证（可在 `src-tauri/tauri.conf.json` 的 `bundle.macOS` 部分补充账号与证书信息）

## 注意

- 因为涉及到按键模拟：Windows/Linux 需以管理员权限运行；macOS 下无需 sudo
- 仅支持在 macOS/Windows 环境下播放音乐自动激活游戏窗口；Linux 下需要手动切换到游戏窗口
- TXT 谱仅支持 Sky Studio 导出的未加密格式，其他格式请自行转换
- 为避免版权争议，该项目不提供任何谱子，请自行寻找