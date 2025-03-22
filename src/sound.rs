//! Module for handling audio playback, including music and sound effects.
//! Uses the `rodio` crate to decode and play embedded audio files.

use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

// -------------------------------------------------------------------
// Audio assets embedded into the binary.
const MUSIC_A_GB: &[u8] = include_bytes!("../resources/music/music-a-gb.mp3");
const MUSIC_A: &[u8] = include_bytes!("../resources/music/music-a.mp3");
const MUSIC_B: &[u8] = include_bytes!("../resources/music/music-b.mp3");

const ROT: &[u8] = include_bytes!("../resources/sfx/rot.wav");
const MOV: &[u8] = include_bytes!("../resources/sfx/mov.wav");
const DROP: &[u8] = include_bytes!("../resources/sfx/drop.wav");
const LOCK: &[u8] = include_bytes!("../resources/sfx/lock.wav");
const PAUSE: &[u8] = include_bytes!("../resources/sfx/pause.wav");
const LINE: &[u8] = include_bytes!("../resources/sfx/line.wav");

// Music list now contains a tuple of song bytes and the panic mode speed factor.
const MUSIC_LIST: [(&[u8], f32); 3] = [(MUSIC_A_GB, 1.5), (MUSIC_A, 2.0), (MUSIC_B, 1.25)];
const SFX_LIST: [&[u8]; 6] = [ROT, MOV, DROP, LOCK, PAUSE, LINE];

#[allow(dead_code)]
pub struct MusicManager {
    mus_stream: OutputStream,
    mus_stream_hndl: OutputStreamHandle,
    pub mus_sink: Sink,
    pub mus_track: u32,
    sfx_sinks: [Sink; 4],
    pub muted: bool,
    pub paused: bool,
    pub panic: bool,
}

impl MusicManager {
    /// Creates a new music manager with initialized audio streams.
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let mscsink = Sink::try_new(&stream_handle).unwrap();
        let sfxsinks: [Sink; 4] = [
            Sink::try_new(&stream_handle).unwrap(),
            Sink::try_new(&stream_handle).unwrap(),
            Sink::try_new(&stream_handle).unwrap(),
            Sink::try_new(&stream_handle).unwrap(),
        ];
        MusicManager {
            mus_stream: stream,
            mus_stream_hndl: stream_handle,
            mus_sink: mscsink,
            mus_track: 0,
            sfx_sinks: sfxsinks,
            muted: false,
            paused: false,
            panic: false,
        }
    }

    /// Plays the current music track in a loop.
    pub fn play_song(&mut self) {
        self.mus_sink.clear();
        let track_index: usize = (self.mus_track % MUSIC_LIST.len() as u32) as usize;
        let track_data: &[u8] = MUSIC_LIST[track_index].0;
        let cursor: Cursor<&[u8]> = Cursor::new(track_data);
        let source: rodio::source::Repeat<Decoder<Cursor<&[u8]>>> =
            Decoder::new(cursor).unwrap().repeat_infinite();
        self.mus_sink.append(source);
        if !self.muted {
            self.mus_sink.set_volume(0.5);
        }
        self.mus_sink.play();
        if self.panic {
            self.mus_sink.set_speed(MUSIC_LIST[track_index].1);
        }
        self.mus_track += 1;
    }

    /// Plays a sound effect identified by `sfx_id`.
    pub fn play_sfx(&mut self, sfx_id: u32) {
        self.sfx_sinks[0].clear();
        let track_index: usize = (sfx_id % SFX_LIST.len() as u32) as usize;
        let track_data: &[u8] = SFX_LIST[track_index];
        let cursor: Cursor<&[u8]> = Cursor::new(track_data);
        let source: Decoder<Cursor<&[u8]>> = Decoder::new(cursor).unwrap();
        self.sfx_sinks[0].append(source);
        if !self.muted {
            self.sfx_sinks[0].set_volume(0.5);
        }
        self.sfx_sinks[0].play();
    }

    /// Toggles the panic mode, which adjusts the music playback speed.
    pub fn toggle_panic(&mut self) {
        self.panic = !self.panic;
        let track_index: usize = ((self.mus_track - 1) % MUSIC_LIST.len() as u32) as usize;
        if self.panic {
            self.mus_sink.set_speed(MUSIC_LIST[track_index].1);
        } else {
            self.mus_sink.set_speed(1.0);
        }
    }

    /// Mutes or unmutes the audio.
    pub fn mute(&mut self) {
        if self.muted {
            self.mus_sink.set_volume(0.5);
            self.sfx_sinks[0].set_volume(0.5);
        } else {
            self.mus_sink.set_volume(0.0);
            self.sfx_sinks[0].set_volume(0.0);
        }
        self.muted = !self.muted;
    }

    /// Pauses or resumes the music.
    pub fn pause(&mut self) {
        if self.paused {
            self.mus_sink.play();
        } else {
            self.mus_sink.pause();
        }
        self.paused = !self.paused;
    }

    /// Resets the music manager to its initial state.
    pub fn reset(&mut self) {
        self.mus_sink.clear();
        self.sfx_sinks[0].clear();
        self.mus_sink.set_speed(1.0);
        self.mus_track = 0;
        self.panic = false;
    }
}
