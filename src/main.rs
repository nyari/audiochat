use sdl2::{self};
use wavedata::{
    self,
    units::{Amplitude, Frequency, Proportion, Time},
};

struct SampleableWrapper<T>(T)
where
    T: wavedata::sampling::Sampleable;

impl<T> wavedata::sampling::Sampleable for SampleableWrapper<T>
where
    T: wavedata::sampling::Sampleable,
{
    fn sample_into_f32(
        &mut self,
        out: wavedata::sampling::SamplesMut,
        rate: wavedata::sampling::SamplingRate,
    ) {
        self.0.sample_into_f32(out, rate)
    }
}

impl<T> sdl2::audio::AudioCallback for SampleableWrapper<T>
where
    T: wavedata::sampling::Sampleable,
{
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.0.sample_into_f32(
            wavedata::sampling::SamplesMut(out),
            wavedata::sampling::SamplingRate::new(44100),
        );
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let audio_specification = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device_constructor = |_spec| {
        let carrier_signal = wavedata::sampling::WaveSampler::new(wavedata::waves::Sine::new(
            Frequency::new(20000.0),
            Time::zero(),
            Amplitude::new(1.0),
        ));
        let data_signal =
            wavedata::sampling::SignalSampler::new(wavedata::signals::enc::am::NRZ::new(
                wavedata::signals::enc::am::NRZConsts::new(
                    Frequency::new(10.0),
                    Proportion::new(0.25),
                    (Amplitude::new(1.0), Amplitude::new(0.0)),
                ),
                wavedata::encodings::enc::nrz::Parameters::new(
                    "Nagyon szeretlek angyalom <3 <3 <3"
                        .as_bytes()
                        .iter()
                        .map(|x| x.clone())
                        .collect(),
                    4,
                ),
            ));
        let composite_sampler = wavedata::sampling::CompositeSampler::new(
            carrier_signal,
            data_signal,
            |input, output| {
                *output = input.0 * input.1;
            },
        );

        SampleableWrapper(composite_sampler)
    };

    let device = audio_subsystem
        .open_playback(None, &audio_specification, device_constructor)
        .unwrap();

    device.resume();

    std::thread::sleep(std::time::Duration::from_millis(15000));
}
