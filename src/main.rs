use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::time::Duration;

enum Waveform {
    Sine,
    Square,
    Sawtooth,
}

struct Oscillator {
    frequency: f32,
    amplitude: f32,
    waveform: Waveform,
    speed: f32,
    phase: f32,
}

impl Oscillator {
    fn new(frequency: f32, amplitude: f32, waveform: Waveform, speed: f32, phase: f32) -> Self {
        Self {
            frequency,
            amplitude,
            waveform,
            speed,
            phase,
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let period = 1.0 / (self.frequency * self.speed);
        self.phase = (self.phase + 1.0 / sample_rate * (2.0 * PI / period)).fract();
        match self.waveform {
            Waveform::Sine => self.amplitude * (2.0 * PI * self.phase).sin(),
            Waveform::Square => self.amplitude * (2.0 * PI * self.phase).sin().signum(),
            Waveform::Sawtooth => (2.0 * self.amplitude / PI) * (self.frequency * PI * self.phase).sin().asin(),
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");
    let config = device.default_output_config().expect("Failed to get default output config");
    let sample_rate = config.sample_rate().0 as f32;

    let mut oscillators = vec![
        Oscillator::new(55.0, 0.3, Waveform::Square, 1.0, 0.0),
        Oscillator::new(110.0, 0.2, Waveform::Sawtooth, 1.0, 0.0),
        Oscillator::new(220.0, 0.1, Waveform::Sine, 1.0, 0.0),
    ];

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = oscillators.iter_mut().map(|o| o.next_sample(sample_rate)).sum();
            }
        },
        |err| eprintln!("An error occurred on the audio stream: {}", err),
        None,
    ).unwrap();
    stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(30));
}
