# simple-synth

![](img/simple-synth.jpg)

This is an experimental project that implements a basic software synthesizer in Rust using the CPAL library. The synthesizer can generate three types of waveforms (sine, square, and sawtooth) and allows you to configure the frequency, amplitude, speed, and phase of each oscillator.

---
## Usage
To use the synthesizer, you'll need to install Rust and Cargo. You can then clone the repository and run the following command to build and run the project:

```
cargo run
```

This will start the synthesizer and play a basic chord using three oscillators with different frequencies, amplitudes, waveforms, speeds, and phases.

```
let mut oscillators = vec![
    Oscillator::new(24.0, 0.01, Waveform::Square, 1.0, 0.0),
    Oscillator::new(5.0, 0.02, Waveform::Sawtooth, 1.0, 0.0),
    Oscillator::new(40.0, 0.01, Waveform::Sine, 1.0, 0.0),
];
```

You can modify the parameters of the oscillators in real time by entering values separated by spaces with the format "_**index frequency amplitude waveform speed phase**_" 
```
index frequency amplitude waveform speed phase
   |      |        |        |        |     |
   1      82      0.05    square     1     0
```
---

> **Warning**
> This synthesizer starts playing sound immediately after being executed. Make sure the volume of your speakers or headphones is not too loud


