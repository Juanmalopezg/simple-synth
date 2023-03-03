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
Oscillator::new(55.0, 0.01, Waveform::Square, 1.0, 0.0),
Oscillator::new(110.0, 0.01, Waveform::Sawtooth, 1.0, 0.0),
Oscillator::new(220.0, 0.01, Waveform::Sine, 1.0, 0.0),
```

You can modify the parameters of the oscillators by editing the **_create_oscillator()_** function in the **_main.rs_** file. You can also add more oscillators by creating additional instances of the _**Oscillator**_ struct and adding them to the _**oscillators**_ vector.

---

> **Warning**
> This synthesizer starts playing sound immediately after being executed. Make sure the volume of your speakers or headphones is not too loud


