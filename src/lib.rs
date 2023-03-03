use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::time::Duration;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

// Definimos una estructura para los parámetros de un oscilador
struct OscillatorParams {
    frequency: f32,
    amplitude: f32,
    waveform: Waveform,
    speed: f32,
    phase: f32,
}

// Definimos una estructura para los osciladores
struct Oscillator {
    params: OscillatorParams,
}

// Definimos una enumeración para los tipos de onda
enum Waveform {
    Sine,
    Square,
    Sawtooth,
}

impl Oscillator {
    fn new(params: OscillatorParams) -> Self {
        Self {
            params,
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let period = 1.0 / (self.params.frequency * self.params.speed);
        let phase_increment = 1.0 / sample_rate * (2.0 * PI / period);
        self.params.phase = (self.params.phase + phase_increment) % 1.0;
        let phase = self.params.phase;
        let sample = match self.params.waveform {
            Waveform::Sine => self.params.amplitude * ((2.0 * PI * phase)).sin(),
            Waveform::Square => {
                if (2.0 * PI * phase).sin() > 0.0 {
                    self.params.amplitude
                } else {
                    -self.params.amplitude
                }
            }
            Waveform::Sawtooth => {
                (2.0 * self.params.amplitude / PI) * (self.params.frequency * PI * phase).sin().asin()
            }
        };
        sample
    }
}

fn main() {
    // Configuramos el dispositivo de salida
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");
    let config = device.default_output_config().expect("Failed to get default output config");
    let sample_rate = config.sample_rate().0 as f32;

    // Creamos los osciladores
    let oscillator1 = create_oscillator(440.0, 0.1, Waveform::Sine, 1.0, 0.0);
    let oscillator2 = create_oscillator(550.0, 0.1, Waveform::Square, 1.0, 0.0);
    let oscillator3 = create_oscillator(660.0, 0.1, Waveform::Sawtooth, 1.0, 0.0);

    let mut oscillators = vec![oscillator1, oscillator2, oscillator3];

    // Creamos el stream de audio
    let stream = device.build_output_stream(&config.into(), move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for (_i, sample) in data.iter_mut().enumerate() {
            let mut sample_sum = 0.0;
            for oscillator in &mut oscillators {
                let oscillator_sample = oscillator.next_sample(sample_rate);
                sample_sum += oscillator_sample;
            }
            *sample = sample_sum;
        }
    }, |err| eprintln!("An error occurred on the audio stream: {}", err), None, ).unwrap();
    stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(50));
}

fn create_oscillator(frequency: f32, amplitude: f32, waveform: Waveform, speed: f32, phase: f32) -> Oscillator {
    let params = OscillatorParams {
        frequency,
        amplitude,
        waveform,
        speed,
        phase,
    };
    Oscillator::new(params)
}

// Indicamos que nuestra función será exportada al lado de JavaScript
#[wasm_bindgen]
pub fn next_sample(sample_rate: f32, frequency: f32, amplitude: f32, waveform: u32, speed: f32, phase: f32) -> f32 {
    // Convertimos el valor del enumerador de Waveform a un valor real
    let waveform = match waveform {
        0 => Waveform::Sine,
        1 => Waveform::Square,
        2 => Waveform::Sawtooth,
        _ => Waveform::Sine,
    };

    // Creamos el oscilador y calculamos el siguiente valor de muestra
    let mut oscillator = Oscillator::new(OscillatorParams {
        frequency,
        amplitude,
        waveform,
        speed,
        phase,
    });
    oscillator.next_sample(sample_rate)
}

