# accessible-sdh-music

Accessible Subtitles for the Deaf and Hard of Hearing (SDH) music visualizer and audio processing pipeline.

## Mission & Goal

Most automated music visualizers rely strictly on raw waveform aesthetics without meaningful contextual information for Deaf and hard-of-hearing audiences. 

The goal of **accessible-sdh-music** is to provide an open, accessible multimedia tool that translates auditory dynamics, structural song changes, tempo shifts, and lyric descriptions into standardized SDH subtitle tracks and synchronized visual indicators. It aims to bridge the gap between pure audio perception and visual representation, allowing users to experience music through comprehensive sensory feedback.

## Current Progress

- **Rust GUI Engine**: Interactive `eframe`/`egui` desktop application with asynchronous rendering support.
- **Multilingual Font Support**: Integrated CJK font rendering for internationalized lyric mapping.
- **Automated Processing Pipeline**: Shell automation (`render-accessible-track.sh`) connecting audio stems to subtitle mapping.
- **Dynamic Amplitude Scaling**: Cube root amplitude scaling implementation for improved visual dynamic range.
- **SDH Subtitle Mapping**: Working demonstration subtitle file (`demo-sdh.srt`) defining acoustic descriptors, musical cues, and descriptive sound events.

## Roadmap

- **Live Microphone / PulseAudio Capture**: Real-time SDH event tagging and low-latency audio capture on Linux.
- **Configurable Visual Cues**: High-contrast, customizable color palettes and tactile frequency zones.
- **Debian Packaging**: Native `.deb` packaging for streamlined deployment and installation across Debian-based systems.
- **Standardized Export Formats**: Direct rendering export to accessible MP4/MKV video containers with embedded SDH tracks.

## Attribution & Disclaimer

Copyright (c) 2026 Jesse Dylan Mahoney (何孟維). All rights reserved.

This is an independent open-source accessibility project developed by Jesse Dylan Mahoney. It is not affiliated with, endorsed by, or associated with any third-party media or accessibility organizations.

## License

Released under the GNU General Public License v3.0 (GPLv3).
