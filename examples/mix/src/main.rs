use audio_overlay::overlay;
use hound::WavReader;
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

fn main() {
    // Set the framerate.
    let framerate: u32 = 44100;
    // Load the audio clips.
    // Source: https://archive.org/download/NasaApollo11OnboardRecordings/11_highlight_2.ogg
    let src: Vec<i16> = WavReader::open("src.wav")
        .unwrap()
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect::<Vec<i16>>();
    // Source: https://archive.org/download/airship1904/airship1904.ogg
    let mut dst: Vec<i16> = WavReader::open("dst.wav")
        .unwrap()
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect::<Vec<i16>>();

    // // Overlay the audio clips. The src clip will start 1.0 seconds after dst begins.
    overlay(src.as_slice(), &mut dst, 1.0, framerate, true);

    // Play the audio clips. Source: https://docs.rs/rodio/latest/rodio/
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SamplesBuffer::new(1, framerate, dst);
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
