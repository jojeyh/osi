use std::{fmt::Error, sync::{Arc, Mutex}, time::Duration};
use cpal::{traits::{StreamTrait, DeviceTrait, HostTrait}, SampleRate};

pub async fn get_transcription() -> Result<Vec<f32>, Error> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("Error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("No supported config")
        .with_sample_rate(SampleRate(16000));

    let data = Arc::new(Mutex::new(Vec::<f32>::new()));
    let callback_data = Arc::clone(&data);

    let stream = device.build_input_stream(
        &supported_config.into(),
        move |input_data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut data = callback_data.lock().unwrap();
            data.extend_from_slice(input_data);
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
        None,
    ).expect("build_input_stream failed");

    stream.play().expect("Stream play failed.");

    // Sleep for 3 seconds
    std::thread::sleep(Duration::from_secs(3));
    drop(stream);

    let unwrapped_audio = Arc::try_unwrap(data)
        .unwrap_or_else(|_| {
            panic!("Failed to unwrap audio data Arc");
        });
    let audio_data = unwrapped_audio.into_inner()
        .unwrap_or_else(|_| {
            panic!("Failed to unwrap audio data Mutex"); 
        });

    Ok(audio_data)
}