# 🎵 Rust Music Player

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


### 播放界面
界面包含：
- 🎵 标题显示区域
- 📊 播放状态指示
- 📄 当前文件名显示
- 🎛️ 播放控制按钮（播放/暂停/停止）
- 🔊 音量控制（音量+/-）
- 📁 文件选择按钮

## 🎯 使用方法

1. **打开文件**: 点击"打开文件"按钮选择音频文件
2. **播放音乐**: 点击"播放"按钮开始播放
3. **暂停/继续**: 点击"暂停"按钮暂停，再次点击继续播放
4. **停止播放**: 点击"停止"按钮停止播放
5. **音量控制**: 使用"音量+"和"音量-"按钮调节音量

## 📁 项目结构

```
rust-music-player/
├── Cargo.toml          # 依赖配置
├── src/
│   └── main.rs         # 主程序
├── assets/             # 资源文件
│   └── font.ttf        # 中文字体（可选）
├── README.md           # 说明文档
└── .gitignore          # Git忽略文件
```

## 🤝 贡献

欢迎提交 Issues 和 Pull Requests！

1. Fork 这个仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 Pull Request

## 📝 待办事项

- [ ] 播放列表功能
- [ ] 播放进度条
- [ ] 播放历史记录
- [ ] 音乐库管理
- [ ] 歌词显示
- [ ] 快捷键支持
- [ ] 主题切换
- [ ] 系统托盘支持

## 📜 许可证

此项目使用 MIT 许可证。查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Iced](https://github.com/iced-rs/iced) - 优秀的 Rust GUI 框架
- [Rodio](https://github.com/RustAudio/rodio) - 强大的音频处理库
- [rfd](https://github.com/PolyMeilex/rfd) - 跨平台文件对话框

## 📧 联系方式

如果您有任何问题或建议，请通过以下方式联系：

- GitHub Issues: [提交问题](https://github.com/lixaioli/rust-music-player/issues)

---

⭐ 如果这个项目对您有帮助，请给它一个 Star！