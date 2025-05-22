#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

/// Uses the non-blocking `Display` API from in-class grayscale.rs example
use microbit::{
    Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        delay::Delay,
        gpio::Level,
        prelude::*,
        twim::{Frequency, Twim},
    },
};

/// IMU driver for LSM303AGR
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

/// Computes the squared magnitude of the acceleration vector
///
/// # Arguments
///
/// * `x`, `y`, `z` -  Axis acceleration in calibrated units of mG, thousands of a G
///
/// # Returns
///
/// * `i32` - The resulting squared magnitude of the acceleration vector
fn squared_magnitude(x: i32, y: i32, z: i32) -> i32 {
    x * x + y * y + z * z
}

/// The single, center dot pattern
const CENTER_DOT: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
]);

/// The exclamation point pattern
const EXCLAMATION: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 9, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0],
]);

const FALLING_THRESHOLD: i32 = 500 * 500;

/// The main program that:
/// - Displays a single, center dot when the board is not falling
/// - Displays an exclamation point and "yells" when falling (< 0.5g)
#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut delay = Delay::new(board.SYST);

    // Initialize I2C and IMU
    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), Frequency::K100);

    let mut imu = Lsm303agr::new_with_i2c(i2c);
    imu.init().unwrap();
    imu.set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz10)
        .unwrap();

    // Configures the speaker pin as seen in `hello_audio.rs`
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

    // Configures the nonblocking display as seen in `grayscale.rs`
    let mut display = Display::new(board.TIMER1, board.display_pins);

    // Starts with single, centered LED
    display.show(&CENTER_DOT);

    loop {
        // For non-blocking display updates as seen in `grayscale.rs`
        display.handle_display_event();

        // Check for new IMU data
        if imu.accel_status().unwrap().xyz_new_data() {
            let accel = imu.acceleration().unwrap();
            let x = accel.x_mg();
            let y = accel.y_mg();
            let z = accel.z_mg();
            let mag2 = squared_magnitude(x, y, z);

            // Falling: Show exclamation point and yells
            if mag2 < FALLING_THRESHOLD {
                display.show(&EXCLAMATION);
                loop {
                    // The generated sound for "yelling", derived from `hello_audio.rs`
                    speaker.set_high().unwrap();
                    delay.delay_us(500_u16);
                    speaker.set_low().unwrap();
                    delay.delay_us(500_u16);

                    display.handle_display_event();

                    // Re-check if still falling
                    if imu.accel_status().unwrap().xyz_new_data() {
                        let accel = imu.acceleration().unwrap();
                        let x = accel.x_mg();
                        let y = accel.y_mg();
                        let z = accel.z_mg();
                        let mag2 = squared_magnitude(x, y, z);
                        if mag2 >= FALLING_THRESHOLD {
                            display.show(&CENTER_DOT);
                            speaker.set_low().unwrap();
                            break;
                        }
                    }
                }
            } else {
                // Stationary
                display.show(&CENTER_DOT);
                speaker.set_low().unwrap();
            }
        }

        // Poll delay
        delay.delay_us(500_u16);
    }
}
