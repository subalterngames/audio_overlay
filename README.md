# Audio Overlay

Overlay audio samples from one array onto another. You can optionally expand the destination array.

The overlay function can be used for i8, i16, i32, i64, f32, and f64.

## Changelog

[Read this.](changelog.md)

## Example

[Example implementation here.](https://github.com/subalterngames/audio_overlay/tree/main/examples/mix) This example relies on two crates that aren't dependencies of `audio_overlay`: `hound` (to open .wav files and convert them into vecs) and `rodio` (to play the audio sample buffer).

```rust
use rodio::{OutputStream, Sink};
use rodio::buffer::SamplesBuffer;
use hound;
use audio_overlay::overlay;

fn main()
{
    // Set the framerate.
    let framerate: u32 = 44100;

    // Load the audio clips.
    // Source: https://archive.org/download/NasaApollo11OnboardRecordings/11_highlight_2.ogg
    let src: Vec<i16> = hound::WavReader::open("src.wav").unwrap().samples::<i16>().map(|s| s.unwrap()).collect::<Vec<i16>>();
    // Source: https://archive.org/download/airship1904/airship1904.ogg
    let mut dst: Vec<i16> = hound::WavReader::open("dst.wav").unwrap().samples::<i16>().map(|s| s.unwrap()).collect::<Vec<i16>>();

    // Overlay the audio clips. The src clip will start 1.0 seconds after dst begins.
    overlay(src.as_slice(), &mut dst, 1.0, framerate, true);

    // Play the audio clips. Source: https://docs.rs/rodio/latest/rodio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SamplesBuffer::new(1, framerate, dst);
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
```