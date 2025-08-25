# 🎵 洛依音乐播放器

一个使用 Rust 和 Iced GUI 框架构建的现代化音乐播放器。


## ✨ 功能特性

- 🎵 支持多种音频格式：MP3, FLAC, WAV, OGG, M4A, AAC
- 🎛️ 音量控制：可调节音量大小
- ⏯️ 播放控制：播放、暂停、停止
- 📁 文件选择：友好的文件对话框
- 🖥️ 现代化界面：美观的用户界面设计
- 🌐 中文支持：完美显示中文字符
- ⚡ 高性能：基于 Rust 的高效音频处理

## 🚀 快速开始

### 环境要求

- Rust 1.89+ 
- Windows, macOS, 或 Linux

### 安装运行

1. 克隆仓库
```bash
git clone https://github.com/lixaioli/rust-music-player.git
cd rust-music-player
```

2. 编译运行
```bash
cargo run
```

## 🛠️ 技术栈

- **语言**: Rust
- **GUI 框架**: [Iced](https://github.com/iced-rs/iced) 0.13
- **音频处理**: [Rodio](https://github.com/RustAudio/rodio)
- **文件对话框**: [rfd](https://github.com/PolyMeilex/rfd)
- **日志**: env_logger
- **异步运行时**: Tokio

## 📸 界面截图

### 主界面

洛依音乐播放器拥有简洁美观的界面设计，完美支持中文显示。

### 界面特色
- 🎵 优雅的标题显示
- 📊 实时播放状态指示
- 📄 当前播放文件名显示
- 🎛️ 直观的播放控制按钮
- 🔊 便捷的音量调节功能
- 📁 友好的文件选择体验

## 🎯 使用方法

1. **选择音乐**: 点击"打开文件"按钮选择您喜爱的音频文件
2. **开始播放**: 点击"播放"按钮开始享受音乐
3. **暂停/继续**: 随时点击"暂停"按钮控制播放
4. **停止播放**: 点击"停止"按钮结束播放
5. **音量调节**: 使用音量控制按钮调节到最适合的音量

## 📁 项目结构

```
luoyi-music-player/
├── Cargo.toml          # 项目配置
├── src/
│   └── main.rs         # 洛依主程序
├── assets/             # 资源文件
│   └── font.ttf        # 中文字体支持
├── README.md           # 项目说明
└── .gitignore          # Git忽略文件
```

## 🌟 为什么选择洛依？

洛依音乐播放器不仅仅是一个简单的音频播放工具，它体现了：

- **优雅设计**: 简洁现代的用户界面
- **高性能**: 基于 Rust 的高效音频处理
- **跨平台**: 支持 Windows、macOS 和 Linux
- **中文友好**: 完美的中文显示和用户体验

## 🤝 参与贡献

欢迎为洛依音乐播放器贡献代码！

1. Fork 本项目
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 🚀 发展规划

- [ ] 播放列表管理
- [ ] 播放进度显示和拖拽
- [ ] 音乐库整理功能
- [ ] 歌词同步显示
- [ ] 均衡器和音效
- [ ] 键盘快捷键
- [ ] 多主题切换
- [ ] 系统托盘集成
- [ ] 在线音乐支持

## 📜 开源协议

本项目基于 MIT 协议开源。详情请查看 [LICENSE](LICENSE) 文件。

## 🙏 特别感谢

- [Iced](https://github.com/iced-rs/iced) - 强大的 Rust GUI 框架
- [Rodio](https://github.com/RustAudio/rodio) - 优秀的音频处理库
- [rfd](https://github.com/PolyMeilex/rfd) - 跨平台文件对话框库

## 📧 联系我们

有任何问题或建议，欢迎联系：

- GitHub Issues: [提交问题](https://github.com/lixaioli/rust-music-player/issues)

---

⭐ 如果洛依音乐播放器对您有帮助，请给我们一个 Star 支持！

🎵 **洛依 - 让音乐更美好** 🎵