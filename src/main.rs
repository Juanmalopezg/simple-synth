use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");
    let config = device.default_output_config().expect("Failed to get default output config");
    let sample_rate = config.sample_rate().0 as f32;

    let freq1: f32 = 440.0;
    let freq2: f32 = 550.0;
    let freq3: f32 = 660.0;
    let amp1: f32 = 0.1;
    let amp2: f32 = 0.1;
    let amp3: f32 = 0.1;
    let waveform1: Waveform = Waveform::Sine;
    let waveform2: Waveform = Waveform::Square;
    let waveform3: Waveform = Waveform::Sawtooth;
    let speed1: f32 = 1.0;
    let speed2: f32 = 1.0;
    let speed3: f32 = 1.0;


    let mut oscillators = vec![
        Oscillator::new(freq1, amp1, waveform1, speed1),
        Oscillator::new(freq2, amp2, waveform2, speed2),
        Oscillator::new(freq3, amp3, waveform3, speed3),
    ];

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for (_i, sample) in data.iter_mut().enumerate() {
                let mut sample_sum = 0.0;
                for oscillator in &mut oscillators {
                    let oscillator_sample = oscillator.next_sample(sample_rate);
                    sample_sum += oscillator_sample;
                }
                *sample = sample_sum;
            }
        }
        ,
        |err| eprintln!("An error occurred on the audio stream: {}", err),
        None,
    )
        .unwrap();

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(5));
}

enum Waveform {
    Sine,
    Square,
    Sawtooth,
}

struct Oscillator {
    frequency: f32,
    amplitude: f32,
    waveform: Waveform,
    phase: f32,
    speed: f32,
}


impl Oscillator {
    fn new(frequency: f32, amplitude: f32, waveform: Waveform, speed: f32) -> Self {
        Self {
            frequency,
            amplitude,
            waveform,
            phase: 0.0,
            speed,
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let period = 1.0 / (self.frequency * self.speed);
        let phase_increment = 1.0 / sample_rate * (2.0 * PI / period);
        self.phase = (self.phase + phase_increment) % 1.0;
        let sample = match self.waveform {
            Waveform::Sine => self.amplitude * ((2.0 * PI * self.phase)).sin(),
            Waveform::Square => {
                if (2.0 * PI * self.phase).sin() > 0.0 {
                    self.amplitude
                } else {
                    -self.amplitude
                }
            }
            Waveform::Sawtooth => {
                (2.0 * self.amplitude / PI) * (self.frequency * PI * self.phase).sin().asin()
            }
        };
        sample
    }
}
