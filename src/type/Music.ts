import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isTauri } from "@/lib/utils";

interface Note {
  time: number; // 时间
  key: string; // 音符
}

class Music {
  name: string; // 曲谱名称
  author: string; // 作者
  transcribedBy: string; // 谱曲者
  bpm: number; // 每分钟节拍数
  pitchLevel: number; // 音高
  songNotes: Note[]; // 音符
  pendingTimeouts: ReturnType<typeof setTimeout>[] = []; // 待执行的任务
  currentTime: number = 0; // 当前播放时间 单位秒
  duration: number = 0; // 曲谱时长
  handlePlay: ReturnType<typeof setInterval> | null = null; // 播放句柄
  isPlay: boolean = false; // 是否正在播放
  private playLock: boolean = false; // 播放锁，防止重复播放

  constructor(
    name: string,
    author: string,
    transcribedBy: string,
    bpm: number,
    pitchLevel: number,
    songNotes: Note[],
  ) {
    this.name = name;
    this.author = author;
    this.transcribedBy = transcribedBy;
    this.bpm = bpm;
    this.pitchLevel = pitchLevel;
    // 按照时间排序
    songNotes.sort((a, b) => a.time - b.time);
    this.songNotes = songNotes;
    this.duration = songNotes[songNotes.length - 1].time / 1000;
  }

  /**
   * 播放曲谱
   */
  async play() {
    this.isPlay = true;

    if (this.playLock) {
      return;
    }
    this.playLock = true;
    // 尝试自动切换到 Sky.app（macOS 有效）。失败不影响后续流程。
    if (isTauri()) {
      try {
        await invoke("focus_sky_app");
      } catch (e) {
        console.warn("focus_sky_app 调用失败，继续等待失焦", e);
      }
    }
    // 等待窗口失去焦点
    await this.waitLostFocus();

    // 如果在等待的过程中，isPlay 变为 false，说明用户点击了停止按钮，直接返回
    if (!this.isPlay) {
      return;
    }

    // 设置播放句柄
    this.clearPlayHandle();
    this.setPlayHandle();

    // 播放每个音符
    this.clearPendingTimeouts();
    for (const note of this.songNotes) {
      if (note.time < this.currentTime * 1000) {
        continue;
      }
      this.pendingTimeouts.push(
        setTimeout(
          () => {
            // Only invoke Tauri backend when running inside Tauri
            if (isTauri()) {
              invoke("press_key", { key: note.key });
            }
          },
          note.time - this.currentTime * 1000,
        ),
      );
    }

    this.playLock = false;
  }

  /**
   * 停止播放
   */
  pause() {
    this.isPlay = false;
    // 清除定时器
    this.clearPendingTimeouts();
    // 清除播放句柄
    this.clearPlayHandle();
  }
  /**
   * 跳转到指定时间播放
   * @param time 跳转的时间，单位秒
   */
  async seekTo(time: number) {
    // 跳转到指定时间播放
    this.currentTime = time;
    // 清除定时器
    this.clearPendingTimeouts();
    if (this.isPlay) {
      this.play();
    }
  }

  /**
   * 清除播放句柄
   * 如果有播放句柄，清除它
   * 如果没有播放句柄，什么都不做
   */
  private clearPlayHandle() {
    // 清除播放句柄
    if (this.handlePlay) {
      clearInterval(this.handlePlay);
    }
  }

  /**
   * 设置播放句柄
   * 每秒更新一次当前时间
   * 如果当前时间超过曲谱时长，自动暂停并重置当前时间
   */
  private setPlayHandle() {
    this.handlePlay = setInterval(() => {
      this.currentTime += 1;
      if (this.currentTime >= this.duration) {
        this.pause();
        this.currentTime = 0;
      }
    }, 1000);
  }

  /**
   * 清除所有待执行的按键
   */
  private clearPendingTimeouts() {
    for (const timeout of this.pendingTimeouts) {
      clearTimeout(timeout);
    }
  }

  /**
   * 等待窗口失去焦点
   * @returns 等待窗口失去焦点的 Promise
   */
  private async waitLostFocus() {
    // In browser dev server, there's no Tauri window. Skip waiting.
    if (!isTauri()) {
      return Promise.resolve();
    }
    return new Promise<void>((resolve) => {
      const handleFocus = setInterval(async () => {
        // 等待应用窗口失去焦点，且前台应用为 Sky.app
        const tauriWindowFocused = await getCurrentWindow().isFocused();
        let frontName = "";
        let bundleId = "";
        try {
          // 通过后端命令获取当前前台应用信息（仅 macOS 有效）
          const info = await invoke<{ name?: string; bundle_id?: string } | null>(
            "front_app_info",
          );
          frontName = (info?.name ?? "").toLowerCase();
          bundleId = (info?.bundle_id ?? "").toLowerCase();
        } catch (_) {
          // 忽略获取失败，继续轮询
        }

        // 兼容多种名称/ID：Sky、Children of the Light、中文名、以及 tgc.sky
        const namePatterns = ["sky", "children of the light", "光遇", "光·遇"]; 
        const idPatterns = ["tgc.sky", "sky"]; // 预估 Bundle ID 片段
        const matchName = namePatterns.some((p) => frontName.includes(p));
        const matchId = idPatterns.some((p) => bundleId.includes(p));

        // 当本窗口失焦，且前台应用匹配 Sky 时认为切到游戏
        if (!tauriWindowFocused && (matchName || matchId)) {
          clearInterval(handleFocus);
          resolve();
        }
      }, 100);
    });
  }
}

function stringToMusic(jsonString: string): Music | null {
  // 1. 解析 JSON 字符串
  const parsedData = JSON.parse(jsonString)[0];

  // 2. 校验解析后的数据类型，确保数据的完整性和正确性
  if (
    typeof parsedData !== "object" ||
    parsedData === null ||
    typeof parsedData.name !== "string" ||
    typeof parsedData.author !== "string" ||
    typeof parsedData.transcribedBy !== "string" ||
    typeof parsedData.bpm !== "number" ||
    typeof parsedData.pitchLevel !== "number" ||
    !Array.isArray(parsedData.songNotes)
  ) {
    console.error("无效的曲谱数据结构", parsedData);
    console.log(parsedData);
    return null;
  }

  // 3. 校验 songNotes 数组中元素的数据类型
  for (const note of parsedData.songNotes) {
    if (
      typeof note !== "object" ||
      note === null ||
      typeof note.time !== "number" ||
      typeof note.key !== "string"
    ) {
      console.error("无效的音符数据结构", note);
      return null;
    }
  }

  // 3. 创建 Music 对象
  const music = new Music(
    parsedData.name,
    parsedData.author,
    parsedData.transcribedBy,
    parsedData.bpm,
    parsedData.pitchLevel,
    parsedData.songNotes,
  );
  return music;
}

export { stringToMusic, Music };
