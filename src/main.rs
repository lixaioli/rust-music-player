use iced::widget::{button, column, text, row, container};
use iced::{Element, Task, Color, Background, Size, Font, Length, Border};
use iced::border::Radius;
use rodio::{Decoder, Sink, OutputStreamBuilder};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;
use log::{info, error, warn};

#[derive(Debug, Clone)]
pub enum Message {
    PlayPause,
    Stop,
    OpenFile,
    FileSelected(PathBuf),
    VolumeUp,
    VolumeDown,
}

pub struct MusicPlayer {
    is_playing: bool,
    current_track: Option<String>,
    sink: Option<Sink>,
    _stream_handle: Option<Box<dyn std::any::Any + Send>>,
    volume: f32,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_track: None,
            sink: None,
            _stream_handle: None,
            volume: 0.5,
        }
    }
}

impl MusicPlayer {
    fn new() -> (Self, Task<Message>) {
        info!("初始化音乐播放器");
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlayPause => {
                if let Some(sink) = &self.sink {
                    if self.is_playing {
                        sink.pause();
                        self.is_playing = false;
                        info!("音乐已暂停");
                    } else {
                        sink.play();
                        self.is_playing = true;
                        info!("音乐继续播放");
                    }
                } else {
                    warn!("请先选择音乐文件");
                }
            }
            Message::Stop => {
                if let Some(sink) = &self.sink {
                    sink.stop();
                    self.is_playing = false;
                    info!("音乐已停止");
                }
            }
            Message::OpenFile => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("选择音频文件")
                            .add_filter("音频文件", &["mp3", "wav", "flac", "ogg", "m4a", "aac"])
                            .pick_file()
                            .await
                    },
                    |file_handle| {
                        if let Some(file_handle) = file_handle {
                            Message::FileSelected(file_handle.path().to_path_buf())
                        } else {
                            Message::Stop
                        }
                    },
                );
            }
            Message::FileSelected(path) => {
                self.load_audio_file(path);
            }
            Message::VolumeUp => {
                self.volume = (self.volume + 0.1).min(1.0);
                if let Some(sink) = &self.sink {
                    sink.set_volume(self.volume);
                }
                info!("音量调整至: {:.1}%", self.volume * 100.0);
            }
            Message::VolumeDown => {
                self.volume = (self.volume - 0.1).max(0.0);
                if let Some(sink) = &self.sink {
                    sink.set_volume(self.volume);
                }
                info!("音量调整至: {:.1}%", self.volume * 100.0);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let status_text = if self.is_playing {
            "正在播放中..."
        } else if self.current_track.is_some() {
            "已暂停"
        } else {
            "未选择文件"
        };

        let track_name = self.current_track
            .as_deref()
            .unwrap_or("请选择一个音频文件开始播放");

        let play_button_text = if self.is_playing {
            "暂停"
        } else {
            "播放"
        };

        // 播放控制按钮
        let controls = row![
            button(text(play_button_text).size(16))
                .padding([12, 24])
                .style(|theme, status| {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.2, 0.6, 1.0))),
                        text_color: Color::WHITE,
                        border: Border::default().rounded(Radius::from(6)),
                        ..button::primary(theme, status)
                    }
                })
                .on_press(Message::PlayPause),
            
            button(text("停止").size(16))
                .padding([12, 24])
                .style(|theme, status| {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.8, 0.3, 0.3))),
                        text_color: Color::WHITE,
                        border: Border::default().rounded(Radius::from(6)),
                        ..button::primary(theme, status)
                    }
                })
                .on_press(Message::Stop),
            
            button(text("打开文件").size(16))
                .padding([12, 24])
                .style(|theme, status| {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.3, 0.7, 0.3))),
                        text_color: Color::WHITE,
                        border: Border::default().rounded(Radius::from(6)),
                        ..button::primary(theme, status)
                    }
                })
                .on_press(Message::OpenFile),
        ]
        .spacing(15);

        // 音量控制
        let volume_controls = row![
            button(text("音量-").size(14))
                .padding([8, 16])
                .style(|theme, status| {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.6, 0.6, 0.6))),
                        text_color: Color::WHITE,
                        border: Border::default().rounded(Radius::from(4)),
                        ..button::secondary(theme, status)
                    }
                })
                .on_press(Message::VolumeDown),
            
            container(
                text(format!("音量: {:.0}%", self.volume * 100.0)).size(18)
            )
            .padding([8, 16])
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                border: Border::default().rounded(Radius::from(4)),
                ..Default::default()
            }),
            
            button(text("音量+").size(14))
                .padding([8, 16])
                .style(|theme, status| {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.6, 0.6, 0.6))),
                        text_color: Color::WHITE,
                        border: Border::default().rounded(Radius::from(4)),
                        ..button::secondary(theme, status)
                    }
                })
                .on_press(Message::VolumeUp),
        ]
        .spacing(15)
        .align_y(iced::Alignment::Center);

        // 主界面
        container(
            column![
                // 标题
                text("🎵 洛依音乐播放器")
                    .size(40)
                    .color(Color::from_rgb(0.2, 0.3, 0.7)),
                
                // 状态显示
                text(status_text)
                    .size(22)
                    .color(Color::from_rgb(0.4, 0.4, 0.4)),
                
                // 当前文件名
                container(
                    text(track_name)
                        .size(16)
                        .color(Color::from_rgb(0.1, 0.1, 0.1))
                )
                .padding(10)
                .style(|_theme| container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.98))),
                    border: Border {
                        color: Color::from_rgb(0.8, 0.8, 0.9),
                        width: 1.0,
                        radius: Radius::from(6.0),
                    },
                    ..Default::default()
                }),
                
                controls,
                volume_controls,
                
                // 系统信息
                text("测试中文显示功能")
                .size(12)
                .color(Color::from_rgb(0.2, 0.7, 0.2)),
            ]
            .spacing(30)
            .padding(40)
            .align_x(iced::Alignment::Center)
        )
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 1.0))),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.8),
                width: 2.0,
                radius: Radius::from(12.0),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

impl MusicPlayer {
    fn load_audio_file(&mut self, path: PathBuf) {
        info!("尝试加载音频文件: {:?}", path);
        
        match self.create_audio_sink() {
            Ok((sink, stream_handle)) => {
                match self.decode_audio_file(&path) {
                    Ok(source) => {
                        sink.append(source);
                        sink.set_volume(self.volume);
                        
                        self.sink = Some(sink);
                        self._stream_handle = Some(Box::new(stream_handle));
                        self.current_track = Some(
                            path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                        );
                        self.is_playing = false;
                        info!("音频文件加载成功: {:?}", self.current_track);
                    }
                    Err(e) => {
                        error!("音频文件解码失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("音频输出初始化失败: {}", e);
            }
        }
    }

    fn create_audio_sink(&self) -> Result<(Sink, Box<dyn std::any::Any + Send>)> {
        let stream_handle = OutputStreamBuilder::open_default_stream()
            .map_err(|e| anyhow::anyhow!("创建音频流失败: {}", e))?;
        let sink = Sink::connect_new(&stream_handle.mixer());
        Ok((sink, Box::new(stream_handle)))
    }

    fn decode_audio_file(&self, path: &PathBuf) -> Result<Decoder<BufReader<File>>> {
        let file = File::open(path)
            .map_err(|e| anyhow::anyhow!("无法打开文件: {}", e))?;
        let buf_reader = BufReader::new(file);
        Decoder::new(buf_reader)
            .map_err(|e| anyhow::anyhow!("音频解码失败: {}", e))
    }
}

#[tokio::main]
async fn main() -> iced::Result {
    // 初始化日志
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("启动洛依音乐播放器 v1.0.0");
    
    // 创建应用程序并设置默认字体
    let app = iced::application("🎵 洛依音乐播放器", MusicPlayer::update, MusicPlayer::view)
        .window_size(Size::new(800.0, 600.0))
        .theme(|_| iced::Theme::Light)
        .default_font(Font::with_name("Microsoft YaHei")); // 设置默认字体
    
    // 运行应用
    info!("使用微软雅黑字体显示中文");
    app.run_with(MusicPlayer::new)
}