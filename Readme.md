# Simple-Synth

This is an experimental project that implements a basic software synthesizer in Rust using the CPAL library. The synthesizer can generate three types of waveforms (sine, square, and sawtooth) and allows you to configure the frequency, amplitude, speed, and phase of each oscillator.

## Usage
To use the synthesizer, you'll need to install Rust and Cargo. You can then clone the repository and run the following command to build and run the project:

```
cargo run
```

This will start the synthesizer and play a basic chord using three oscillators with different frequencies, amplitudes, and waveforms.

```
let oscillator1 = create_oscillator(440.0, 0.1, Waveform::Sine, 1.0, 0.0);
let oscillator2 = create_oscillator(550.0, 0.1, Waveform::Square, 1.0, 0.0);
let oscillator3 = create_oscillator(660.0, 0.1, Waveform::Sawtooth, 1.0, 0.0);
```

You can modify the parameters of the oscillators by editing the **_create_oscillator()_** function in the **_main.rs_** file. You can also add more oscillators by creating additional instances of the _**Oscillator**_ struct and adding them to the _**oscillators**_ vector.


