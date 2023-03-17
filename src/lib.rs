//! Overlay audio samples from one array onto another. You can optionally expand the destination array.
//! 
//! The overlay function can be used for i8, i16, i32, i64, and f32.
//! 
//! # Example
//! 
//! ```rust
//! use rodio::{OutputStream, Sink};
//! use rodio::buffer::SamplesBuffer;
//! use hound;
//! use audio_overlay::overlay;
//! 
//! fn main()
//! {
//!     // Set the framerate.
//!     let framerate: u32 = 44100;
//!     // Load the audio clips.
//!     // Source: https://archive.org/download/NasaApollo11OnboardRecordings/11_highlight_2.ogg
//!     let src: Vec<i16> = hound::WavReader::open("src.wav").unwrap().samples::<i16>().map(|s| s.unwrap()).collect::<Vec<i16>>();
//!     // Source: https://archive.org/download/airship1904/airship1904.ogg
//!     let mut dst: Vec<i16> = hound::WavReader::open("dst.wav").unwrap().samples::<i16>().map(|s| s.unwrap()).collect::<Vec<i16>>();
//! 
//!     // Overlay the audio clips. The src clip will start 1.0 seconds after dst begins.
//!     overlay(src.as_slice(), &mut dst, 1.0, framerate, true);
//! 
//!     // Play the audio clips. Source: https://docs.rs/rodio/latest/rodio
//!     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//!     let source = SamplesBuffer::new(1, framerate, dst);
//!     let sink = Sink::try_new(&stream_handle).unwrap();
//!     sink.append(source);
//!     sink.sleep_until_end();
//! }
//! ```

use std::ops::Add;
use std::cmp::PartialOrd;

/// Overlay audio samples from one array onto another. You can optionally expand the destination array.
/// 
/// This function can be used for i8, i16, i32, i64, and f32.
/// 
/// This function assumes that both the source and destination arrays are a single channel of audio and have the same framerate and sample width.
/// 
/// For multi-channel audio, run `overlay()` for each channel.
/// 
/// Audio mixing algorithm source: <https://github.com/python/cpython/blob/main/Modules/audioop.c#L1083>
/// 
/// # Arguments
/// 
/// * `src` - A slice of type T. This array will be overlaid into `dst`.
/// * `dst` - A mutable vec of type T. This will be modified, with `src` being overlaid into `dst`.
/// * `time` - The start time in seconds at which `src` should be overlaid into `dst`.
/// * `framerate` - The framerate of `src` and `dst`, e.g. 44100. This will be used to convert `time` into an index value.
/// * `push` - Often, the end time of `src` will exceed the end time of `dst`. If `push == true`, samples from `src` past the original end time of `dst` will be pushed to `dst`, lengthening the waveform. If `push == false`, this function will end at the current length of `dst` and won't modify its length.
pub fn overlay<T, U>(src: &[T], dst: &mut Vec<T>, time: f64, framerate: u32, push: bool)
    where T: Copy + Add + PartialOrd + ValueBounds<T> + CastableUp<T, U> + From<u8>,
    T: Add<Output = T>,
    U: Copy + PartialOrd + ValueBounds<U> + CastableDown<U, T> + Add,
    U: Add<Output = U>
{
    // Get the start index.
    let mut index: usize = (time * framerate as f64) as usize;
    let len: usize = dst.len();
    // Get the minimum and maximum values.
    let min: U = U::min();
    let max: U = U::max();
    let zero: T = T::from(0);
    let mut pushing = false;
    for &v in src
    {
        // Append instead of overlaying.
        if pushing
        {
            dst.push(v);
            continue;
        }
        // If the index is greater than the length of dst, then we need to either stop here or start pushing.
        if index >= len
        {
            // Don't push it.
            if !push
            {
                return;
            }
            pushing = true;
            dst.push(v);
            continue;
        }
        // If there is no data at this index, set it to v rather than doing a lot of casting.
        if dst[index] == zero
        {
            dst[index] = v;
        }
        // Overlay the sample.
        else 
        {
            dst[index] = U::cast(fbound(dst[index].cast() + v.cast(), min, max));
        }
        // Increment the index.
        index += 1;
    }
}

// Source: https://github.com/python/cpython/blob/65fb7c4055f280caaa970939d16dd947e6df8a8d/Modules/audioop.c#L44
fn fbound<T>(value: T, min: T, max: T) -> T
    where T: PartialOrd
{
    if value > max
    {
        max
    }
    else if value < min
    {
        min
    }
    else 
    {
        value
    }
}

/// This is used by `overlay()` to get the minimum and maximum values of a given type.
pub trait ValueBounds<T>
    where T: Copy + PartialOrd
{
    /// Returns the minimum value of type T.
    fn min() -> T;
    /// Returns the maximum value of type T.
    fn max() -> T;
}

impl ValueBounds<i8> for i8
{
    fn min() -> i8
    {
        i8::MIN
    }

    fn max() -> i8
    {
        i8::MAX
    }
}

impl ValueBounds<i16> for i16
{
    fn min() -> i16
    {
        i16::MIN
    }

    fn max() -> i16
    {
        i16::MAX
    }
}

impl ValueBounds<i32> for i32
{
    fn min() -> i32
    {
        i32::MIN
    }

    fn max() -> i32
    {
        i32::MAX
    }
}

impl ValueBounds<i64> for i64
{
    fn min() -> i64
    {
        i64::MIN
    }

    fn max() -> i64
    {
        i64::MAX
    }
}

impl ValueBounds<f32> for f32
{
    fn min() -> f32
    {
        f32::MIN
    }

    fn max() -> f32
    {
        f32::MAX
    }
}

/// This is used by `overlay()` to cast a value to a higher type, e.g. i16 to i32, to prevent overflow errors.
pub trait CastableUp<T, U>
    where T: Copy + PartialOrd,
    U: Copy + PartialOrd
{
    /// Convert this value to type U.
    fn cast(self) -> U;
}

impl CastableUp<i8, i16> for i8
{
    fn cast(self) -> i16 
    {
        self as i16
    }
}

impl CastableUp<i16, i32> for i16
{
    fn cast(self) -> i32 
    {
        self as i32
    }
}

impl CastableUp<i32, i64> for i32
{
    fn cast(self) -> i64 
    {
        self as i64
    }
}

impl CastableUp<i64, i128> for i64
{
    fn cast(self) -> i128 
    {
        self as i128
    }
}

impl CastableUp<f32, f64> for f32
{
    fn cast(self) -> f64 
    {
        self as f64
    }
}

/// This is used by `overlay()` to cast a value to a lower type, e.g. i32 to i16, once we're done dealing with potential overflow errors.
pub trait CastableDown<T, U>
    where T: Copy + PartialOrd,
    U: Copy + PartialOrd
{
    /// Convert the value to type U.
    fn cast(value: T) -> U;
}

impl CastableDown<i16, i8> for i16
{
    fn cast(value: i16) -> i8
    {
        value as i8
    }
}

impl CastableDown<i32, i16> for i32
{
    fn cast(value: i32) -> i16 
    {
        value as i16
    }
}

impl CastableDown<i64, i32> for i64
{
    fn cast(value: i64) -> i32 
    {
        value as i32
    }
}

impl CastableDown<i128, i64> for i128
{
    fn cast(value: i128) -> i64 
    {
        value as i64
    }
}

impl CastableDown<f64, f32> for f64
{
    fn cast(value: f64) -> f32 
    {
        value as f32
    }
}