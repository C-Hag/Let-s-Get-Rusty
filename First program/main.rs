use egui::{containers::*, *};
use rodio::Source;
use std::time::Duration;
use minimp3::{Decoder, Frame};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;

// Struct to represent an audio file entry in the media library
struct AudioFile {
    file_path: String,
    metadata: Metadata,
}

// Struct to represent metadata for an audio file
struct Metadata {
    title: String,
    artist: String,
    // Add more metadata fields as needed
}

// Struct to manage the media library
struct MediaLibrary {
    entries: Vec<AudioFile>,
}

impl MediaLibrary {
    fn new() -> Self {
        MediaLibrary { entries: Vec::new() }
    }

    fn add_entry(&mut self, file_path: String, metadata: Metadata) {
        let entry = AudioFile { file_path, metadata };
        self.entries.push(entry);
    }

    // Implement functions for removing and editing entries as needed
}

// Struct to manage audio playback and equalization
struct PlayerState {
    audio_file: Option<Arc<Mutex<Decoder<BufReader<File>>>>>,
    playback_handle: Option<rodio::Sink>,
    is_playing: bool,
    progress: f32,
    waveform: Option<Waveform>,
    media_library: MediaLibrary,
    equalizer: Equalizer,
}

// Struct to represent the waveform data
struct Waveform {
    data: Vec<f32>,
}

// Struct to manage the equalizer settings
struct Equalizer {
    bass_gain: f32,
    midrange_gain: f32,
    treble_gain: f32,
}

impl PlayerState {
    fn new() -> Self {
        PlayerState {
            audio_file: None,
            playback_handle: None,
            is_playing: false,
            progress: 0.0,
            waveform: None,
            media_library: MediaLibrary::new(),
            equalizer: Equalizer {
                bass_gain: 0.0,
                midrange_gain: 0.0,
                treble_gain: 0.0,
            },
        }
    }

    // Function to load an audio file and compute the waveform
    fn load_audio_file(&mut self, file_path: &str) {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            let decoder = Decoder::new(reader);

            let waveform = self.compute_waveform(&decoder);

            self.audio_file = Some(Arc::new(Mutex::new(decoder)));
            self.playback_handle = Some(rodio::Sink::new());
            self.is_playing = false;
            self.progress = 0.0;
            self.waveform = Some(waveform);
        }
    }

    // Function to compute the waveform data from the audio file
    fn compute_waveform(&self, decoder: &Decoder<BufReader<File>>) -> Waveform {
        let mut waveform_data = Vec::new();
        let mut samples = Vec::new();

        for frame_result in decoder.frames() {
            if let Ok(Frame { data, .. }) = frame_result {
                for sample in data {
                    samples.push(sample as f32 / i16::MAX as f32);
                }
            }
        }

        // Adjust the buffer size as needed
        let step = samples.len() / 1000;
        for i in (0..samples.len()).step_by(step) {
            let start = i;
            let end = std::cmp::min(i + step, samples.len());
            let slice = &samples[start..end];
            let max_sample = slice.iter().cloned().fold(0.0, f32::max);
            waveform_data.push(max_sample);
        }

        Waveform { data: waveform_data }
    }

    // Function to play or pause audio
    fn play_pause(&mut self) {
        if let Some(audio_file) = &self.audio_file {
            let mut playback_handle = self.playback_handle.as_ref().unwrap().clone();
            if playback_handle.empty() {
                let audio_file = audio_file.lock().unwrap().clone();
                playback_handle.append(audio_file);
                playback_handle.play();
                self.is_playing = true;
            } else if playback_handle.is_paused() {
                playback_handle.play();
                self.is_playing = true;
            } else {
                playback_handle.pause();
                self.is_playing = false;
            }
        }
    }

    // Function to update the progress of audio playback
    fn update_progress(&mut self) {
        if let Some(audio_file) = &self.audio_file {
            let duration = audio_file.lock().unwrap().total_duration();
            let position = self.playback_handle.as_ref().unwrap().total_duration();
            if duration.as_secs() > 0 {
                self.progress = position.as_secs_f32() / duration.as_secs_f32();
            }
        }
    }
}

fn main() {
    let options = epi::Options {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    let player_state = PlayerState::new();
    let player_state = Arc::new(Mutex::new(player_state));

    let audio_player = epi::App::new(options, player_state);

    eframe::run_native(audio_player);
}

impl epi::App for PlayerState {
    fn update(&mut self, ctx: &egui::Context) {
        let mut player_state = self.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Rust Audio Player");

            if ui.button("Load Audio").clicked() {
                // Implement file picker dialog here to load an audio file
                // For simplicity, let's hardcode a file path for now.
                player_state.load_audio_file("path/to/your/audio/file.mp3");
            }

            if let Some(audio_file) = &player_state.audio_file {
                ui.horizontal(|ui| {
                    if ui.button("Play/Pause").clicked() {
                        player_state.play_pause();
                    }

                    ui.label(format!("Progress: {:.2}%", player_state.progress * 100.0));
                });

                player_state.update_progress();

                // Display the waveform if available
                if let Some(waveform) = &player_state.waveform {
                    let size = egui::vec2(600.0, 100.0); // Adjust the size as needed
                    let painter = ui.layer_painter(LayerId::new(Order::Foreground, Id::new(Order::Background, "waveform")));
                    painter.rect_filled(painter.screen_rect(), 0.0, egui::Color32::BLACK);
                    painter.path(|path| {
                        let step = size.x / waveform.data.len() as f32;
                        let mut x = 0.0;
                        let half_size = size.y / 2.0;
                        path.move_to(egui::pos(x, half_size));
                        for &sample in &waveform.data {
                            x += step;
                            let y = half_size - sample * half_size;
                            path.line_to(egui::pos(x, y));
                        }
                    });
                }
            }
        });
    }
}
