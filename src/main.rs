use sdl2::{self};
use wavedata;


struct SampleableWrapper<T>(T) where T: wavedata::sampling::Sampleable + wavedata::waves::Wave;

impl<T> wavedata::sampling::Sampleable for SampleableWrapper<T> 
    where T: wavedata::sampling::Sampleable + wavedata::waves::Wave {
    fn sample_into_f32(&self, out: &mut [f32], rate: wavedata::sampling::SamplingRate) -> wavedata::units::Time {
        self.0.sample_into_f32(out, rate)    
    }
}

impl<T> sdl2::audio::AudioCallback for SampleableWrapper<T> 
    where T: wavedata::sampling::Sampleable + wavedata::waves::Wave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        let offset = self.0.sample_into_f32(out, wavedata::sampling::SamplingRate::new(44100));
        self.0.shift_mut(offset);
    }
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let audio_specification = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };

    let device = audio_subsystem.open_playback(None, &audio_specification, |_spec| {
        SampleableWrapper(wavedata::waves::Sine::new(
            wavedata::units::Frequency::new(100.0),
            wavedata::units::Time::zero(),
            wavedata::units::Amplitude::new(1.0)
        ))
    }).unwrap();

    device.resume();

    std::thread::sleep(std::time::Duration::from_millis(10000));
    
}
