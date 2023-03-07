use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::thread;
use std::io::stdin;
use flume::{Receiver, Sender, unbounded};

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

struct OscillatorParams {
    index: usize,
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
            Waveform::Sawtooth => self.amplitude * ((2.0 / PI) * (self.frequency * PI * self.phase).sin().asin()),
        }
    }
}

fn main() {
    // Create a channel for thread communication
    let (tx, rx): (Sender<OscillatorParams>, Receiver<OscillatorParams>) = unbounded();

    // Starts audio thread
    thread::spawn(move || {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to get default output device");
        let config = device.default_output_config().expect("Failed to get default output config");
        let sample_rate = config.sample_rate().0 as f32;

        let mut oscillators = vec![
            Oscillator::new(20.0, 0.01, Waveform::Square, 1.0, 0.0),
            Oscillator::new(5.0, 0.02, Waveform::Sawtooth, 1.0, 0.0),
            Oscillator::new(40.0, 0.01, Waveform::Sine, 1.0, 0.0),
        ];

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                while let Ok(params) = rx.try_recv() {
                    if let Some(oscillator) = oscillators.get_mut(params.index) {
                        oscillator.frequency = params.frequency;
                        oscillator.amplitude = params.amplitude;
                        oscillator.waveform = params.waveform;
                        oscillator.speed = params.speed;
                        oscillator.phase = params.phase;
                    } else {
                        oscillators.push(Oscillator::new(params.frequency, params.amplitude, params.waveform, params.speed, params.phase));
                    }
                }

                for sample in data.iter_mut() {
                    *sample = oscillators.iter_mut().map(|o| o.next_sample(sample_rate)).sum();
                }
            },
            |err| eprintln!("An error occurred on the audio stream: {}", err),
            None,
        ).unwrap();

        stream.play().unwrap();
        thread::park();
    });

    println!("\n\t\tsimple-synth running!\nEnter values separated by space to modify an oscillator.
    \rUse the following format:\n\tindex frequency amplitude waveform speed phase\n");
    loop {
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();

        let values: Vec<&str> = input_str.trim().split_whitespace().collect();

        if values.len() != 6 {
            println!("Must enter exactly six values separated by spaces.");
            continue;
        }

        if values.len() == 6 {
            let index: usize = match values[0].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let frequency: f32 = match values[1].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let amplitude: f32 = match values[2].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let waveform = match values[3] {
                "sine" => Waveform::Sine,
                "square" => Waveform::Square,
                "saw" => Waveform::Sawtooth,
                _ => continue,
            };

            let speed: f32 = match values[4].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let phase: f32 = match values[5].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            tx.send(OscillatorParams {
                index,
                frequency,
                amplitude,
                waveform,
                speed,
                phase,
            }).unwrap();
        }
    }
}

