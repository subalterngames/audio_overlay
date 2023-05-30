//! Overlay audio samples from one array onto another. You can optionally expand the destination array.
//!
//! The overlay function can be used for i8, i16, i32, i64, and f32.
//!
//! # Example
//!
//! ```rust
//!use audio_overlay::overlay;
//!use hound::WavReader;
//!use rodio::buffer::SamplesBuffer;
//!use rodio::{OutputStream, Sink};
//!
//!// Set the framerate.
//!let framerate: u32 = 44100;
//!// Load the audio clips.
//!// Source: https://archive.org/download/NasaApollo11OnboardRecordings/11_highlight_2.ogg
//!let src: Vec<i16> = WavReader::open("src.wav")
//!.unwrap()
//!.samples::<i16>()
//!.map(|s| s.unwrap())
//!.collect::<Vec<i16>>();
//!// Source: https://archive.org/download/airship1904/airship1904.ogg
//!let mut dst: Vec<i16> = WavReader::open("dst.wav")
//!.unwrap()
//!.samples::<i16>()
//!.map(|s| s.unwrap())
//!.collect::<Vec<i16>>();
//!
//!// // Overlay the audio clips. The src clip will start 1.0 seconds after dst begins.
//!overlay(src.as_slice(), &mut dst, 1.0, framerate, true);
//!
//!// Play the audio clips. Source: https://docs.rs/rodio/latest/rodio/
//!let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//!let source = SamplesBuffer::new(1, framerate, dst);
//!let sink = Sink::try_new(&stream_handle).unwrap();
//!sink.append(source);
//!sink.sleep_until_end();
//! ```

use std::cmp::PartialOrd;

const I8_MAX: i16 = i8::MAX as i16;
const I8_MIN: i16 = i8::MIN as i16;
const I16_MAX: i32 = i16::MAX as i32;
const I16_MIN: i32 = i16::MIN as i32;
const I32_MAX: i64 = i32::MAX as i64;
const I32_MIN: i64 = i32::MIN as i64;
const I64_MAX: i128 = i64::MAX as i128;
const I64_MIN: i128 = i64::MIN as i128;

/// Overlay audio samples from one array onto another. You can optionally expand the destination array.
///
/// This function can be used for i8, i16, i32, i64, f32, and f64.
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
/// * `add` - Often, the end time of `src` will exceed the end time of `dst`. If `add == true`, samples from `src` past the original end time of `dst` will be pushed to `dst`, lengthening the waveform. If `add == false`, this function will end at the current length of `dst` and won't modify its length.
///
/// # Panics
///
/// It is technically possible for this function to panic if the source arrays are of type f32 or f64 because an overlaid value could exceed f32::MIN or f32::MAX, or f64::MIN or f64::MAX, respectively. But this would be a very unusual audio array in the first place. We're assuming that all values in `src` and `dst` are between -1.0 and 1.0.
///
/// For integer types such as i16, the function *won't* panic due to overflow errors because summed values will be cast to a type with a higher bit count, added, and recast as the original type (see [Overlayable]).
pub fn overlay<T, U>(src: &[T], dst: &mut Vec<T>, time: f64, framerate: u32, add: bool)
where
    T: Copy + PartialOrd + Overlayable<T, U> + From<u8>,
    U: Copy + PartialOrd + ValueBounds<U>,
{
    // Get the start index.
    let mut index: usize = (time * framerate as f64) as usize;
    // The current length of dst.
    let len: usize = dst.len();
    // This will be used to fill dst with zeros if needed.
    let zero: T = T::from(0);
    // The start time is after the end of dst.
    if index >= len {
        if add {
            // Add zeros up to the start time.
            dst.extend(vec![zero; index - len]);
            // Add src.
            dst.extend(src.iter().cloned());
        }
        return;
    }
    // Get the minimum and maximum values.
    let min: U = U::min();
    let max: U = U::max();
    for (i, &v) in src.iter().enumerate() {
        // If the index is greater than the length of dst, then we need to either stop here or append.
        if index >= len {
            if add {
                dst.extend(src[i..].iter().cloned());
            }
            return;
        }
        // If there is no data at this index, set it to v rather than doing a lot of casting.
        if dst[index] == zero {
            dst[index] = v;
        }
        // Overlay the sample.
        else {
            dst[index] = T::overlay(dst[index], v, min, max);
        }
        // Increment the index.
        index += 1;
    }
}

// Clamp the value between a min and max.
fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

/// Overlay values onto each other. The values are are added together and clamped to min/max values.
pub trait Overlayable<T, U>
where
    T: Copy + PartialOrd,
    U: Copy + PartialOrd,
{
    /// Add two values together, clamped between min/max values.
    ///
    /// For integer types, we need to cast the values to a higher bit count, e.g. i16 to i32, before adding them, to prevent an overflow error. Values are clamped to the min/max values of the original type, e.g. i16::MAX.
    ///  
    /// For float types, it's assumed that the values are between -1.0 and 1.0. They are added and the sum is clamped to be between -1.0 and 1.0.
    fn overlay(a: T, b: T, min: U, max: U) -> T;
}

impl Overlayable<i8, i16> for i8 {
    fn overlay(a: i8, b: i8, min: i16, max: i16) -> i8 {
        clamp((a + b) as i16, min, max) as i8
    }
}

impl Overlayable<i16, i32> for i16 {
    fn overlay(a: i16, b: i16, min: i32, max: i32) -> i16 {
        clamp((a + b) as i32, min, max) as i16
    }
}

impl Overlayable<i32, i64> for i32 {
    fn overlay(a: i32, b: i32, min: i64, max: i64) -> i32 {
        clamp((a + b) as i64, min, max) as i32
    }
}

impl Overlayable<i64, i128> for i64 {
    fn overlay(a: i64, b: i64, min: i128, max: i128) -> i64 {
        clamp((a + b) as i128, min, max) as i64
    }
}

impl Overlayable<f32, f32> for f32 {
    fn overlay(a: f32, b: f32, min: f32, max: f32) -> f32 {
        clamp(a + b, min, max)
    }
}

impl Overlayable<f64, f64> for f64 {
    fn overlay(a: f64, b: f64, min: f64, max: f64) -> f64 {
        clamp(a + b, min, max)
    }
}

/// This is used by `overlay()` to get the minimum and maximum values of a given type for the purposes of overlaying data.
///
/// For integer types, this is the min/max value of the type one bit count less than this one. For example, `i16::min()` returns `i8::MIN as i16`.
///
/// For float types, the min/max value is -1.0 and 1.0.
pub trait ValueBounds<T>
where
    T: Copy + PartialOrd,
{
    /// Returns the minimum value of type T.
    fn min() -> T;
    /// Returns the maximum value of type T.
    fn max() -> T;
}

impl ValueBounds<i16> for i16 {
    fn min() -> i16 {
        I8_MIN
    }

    fn max() -> i16 {
        I8_MAX
    }
}

impl ValueBounds<i32> for i32 {
    fn min() -> i32 {
        I16_MIN
    }

    fn max() -> i32 {
        I16_MAX
    }
}

impl ValueBounds<i64> for i64 {
    fn min() -> i64 {
        I32_MIN
    }

    fn max() -> i64 {
        I32_MAX
    }
}

impl ValueBounds<i128> for i128 {
    fn min() -> i128 {
        I64_MIN
    }

    fn max() -> i128 {
        I64_MAX
    }
}

impl ValueBounds<f32> for f32 {
    fn min() -> f32 {
        -1.0
    }

    fn max() -> f32 {
        1.0
    }
}

impl ValueBounds<f64> for f64 {
    fn min() -> f64 {
        -1.0
    }

    fn max() -> f64 {
        1.0
    }
}
