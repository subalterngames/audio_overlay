use std::convert::From;
use std::ops::{Add, Div, Mul, Sub};
use std::cmp::PartialOrd;

fn main() 
{
    println!("Hello, world!");
}


/// Overlay audio samples from one array onto another. You can optionally expand the destination array.
/// 
/// This function can be used for any numerical primitive type (i16, f32, etc.).
/// 
/// This function assumes that both the source and destination arrays are a single channel of audio and have the same framerate and sample width.
/// 
/// # Arguments
/// 
/// * `src` - A slice of type T. This array will be overlaid into `dst`.
/// * `dst` - A vec of type T. This will be modified, with `src` being overlaid into `dst`.
/// * `time` - The start time in seconds at which `src` should be overlaid into `dst`.
/// * `framerate` - The framerate of `src` and `dst`, e.g. 44100. This will be used to convert `time` into an index value.
/// * `push` - Often, the end time of `src` will exceed the end time of `dst`. If `push == true`, samples from `src` past the original end time of `dst` will be pushed to `dst`, lengthening the waveform. If `push == false`, this function will end at the current length of `dst` and won't modify its length.
/// 
/// Generic constraints source: https://stackoverflow.com/a/66306889
/// Audio mixing algorithm source: https://github.com/accord-net/framework/blob/development/Sources/Accord.Audio/AudioSourceMixer.cs#L432
pub fn overlay<T>(src: &[T], dst: &mut Vec<T>, time: f64, framerate: u32, push: bool)
    where T: Copy + Add + Sub + Mul + Div + PartialOrd + From<u8> + ValueBounds<T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>
{
    // Get the start index.
    let mut index: usize = (time * framerate as f64) as usize;
    let len: usize = dst.len();
    // Define zero.
    let zero: T = T::from(0);
    // Get the minimum and maximum values.
    let min: T = T::min();
    let max: T = T::max();
    let mut pushing = false;
    for &v in src
    {
        // Append instead of overlaying.
        if pushing
        {
            dst.push(v);
        }
        // If the index is greater than the length of dst, then we need to either stop here or start pushing.
        else if index >= len
        {
            // Don't push it.
            if !push
            {
                return;
            }
            pushing = true;
            dst.push(v);
        }
        // Overlay the sample.
        else 
        {
            if dst[index] < zero && v < zero
            {
                dst[index] = (dst[index] + v) - ((dst[index] * v) / min);
            }
            else if dst[index] > zero && v > zero
            {
                dst[index] = (dst[index] + v) - ((dst[index] * v) / max);
            }
            else 
            {
                dst[index] = dst[index] + v;
            }
        }
        // Increment the index.
        index += 1;
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

impl ValueBounds<u8> for u8
{
    fn min() -> u8
    {
        u8::MIN
    }

    fn max() -> u8
    {
        u8::MAX
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

impl ValueBounds<u16> for u16
{
    fn min() -> u16
    {
        u16::MIN
    }

    fn max() -> u16
    {
        u16::MAX
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

impl ValueBounds<u32> for u32
{
    fn min() -> u32
    {
        u32::MIN
    }

    fn max() -> u32
    {
        u32::MAX
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

impl ValueBounds<u64> for u64
{
    fn min() -> u64
    {
        u64::MIN
    }

    fn max() -> u64
    {
        u64::MAX
    }
}

impl ValueBounds<i128> for i128
{
    fn min() -> i128
    {
        i128::MIN
    }

    fn max() -> i128
    {
        i128::MAX
    }
}

impl ValueBounds<u128> for u128
{
    fn min() -> u128
    {
        u128::MIN
    }

    fn max() -> u128
    {
        u128::MAX
    }
}