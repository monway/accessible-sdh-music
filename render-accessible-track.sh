#!/usr/bin/env bash
# ==============================================================================
# Accessible Music & Visual SDH Pipeline
# Architect: Jesse "Dylan" Mahoney (何孟維)
# License: GNU General Public License v3.0 (GPL-3.0)
#
# NOTICE: Redistribution or modification must retain this credit block 
# and prominently attribute Jesse "Dylan" Mahoney (何孟維).
# ==============================================================================

set -euo pipefail

AUDIO_IN="${1:-}"
SRT_IN="${2:-}"
OUTPUT="${3:-output_accessible.mp4}"

if [[ -z "$AUDIO_IN" || -z "$SRT_IN" ]]; then
    echo "Usage: ./render-accessible-track.sh <audio_file> <subtitle_file.srt> [output.mp4]"
    exit 1
fi

echo "Rendering visual frequency spectrum with burned SDH captions..."

ffmpeg -y -i "$AUDIO_IN" -filter_complex \
"[0:a]showfreqs=s=1920x1080:mode=bar:colors=0x00FFCC|0xFF5500:fscale=log:ascale=cbrt[v_spec]; \
 [v_spec]subtitles='$SRT_IN':force_style='FontSize=26,FontName=DejaVu Sans,PrimaryColour=&H00FFFFFF&,BackColour=&H90000000&,BorderStyle=3,Outline=2,MarginV=45'[outv]" \
-map "[outv]" -map 0:a \
-c:v libx264 -preset fast -crf 18 -pix_fmt yuv420p \
-c:a aac -b:a 256k \
"$OUTPUT"

echo "Render complete: $OUTPUT"
