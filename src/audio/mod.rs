use std::fmt::Error;
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleRate};
use rtrb::RingBuffer;

const SPEECH_BUFFER_COUNT: u8 = 20; 

pub async fn record() -> Result<Vec<f32>, Error> {
    let mut vad = webrtc_vad::Vad::new_with_rate(webrtc_vad::SampleRate::Rate16kHz);
    vad.set_mode(webrtc_vad::VadMode::VeryAggressive);
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("Error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("No supported config")
        .with_sample_rate(SampleRate(16000));

    // let data = Arc::new(Mutex::new(Vec::<f32>::new()));
    // let callback_data = Arc::clone(&data);
    let (mut producer, mut consumer) = RingBuffer::<i16>::new(1024);

    let stream = device.build_input_stream(
        &supported_config.into(),
        move |input_data: &[i16], _: &cpal::InputCallbackInfo| {
            for sample in input_data {
                producer.push(*sample).expect("Failed to push sample to ring buffer");
            }
        },
        move |err| {
            eprintln!("An error occurred on stream: {}", err);
        },
        None,
    ).expect("build_input_stream failed");

    stream.play().expect("Stream play failed.");

    let mut unactive_count = 0;
    let mut speaking = false;
    let mut speech_segment = Vec::<i16>::new();
    loop {
        /*
            TODO This is a dirty hack and should be changed to an algorithm
            that transcribes in short segments and also concatenates those segments 
            checking the results against one another, the choice of length of small vs 
            long segment will be hard to figure out
         */
        if consumer.slots() > 160 {
            let mut audio_frame = vec![0i16; 160];
            for _ in 0..160 {
                match consumer.pop() {
                    Ok(value) => audio_frame.push(value),
                    Err(err) => {
                        println!("Error: {}", err);
                        break;
                    },
                }
            }
            let speech_active = vad.is_voice_segment(&audio_frame)
                .expect("Failed to check voice segment");
            match speech_active {
                true => {
                    match speaking {
                        true => {
                            // Active speech detected and already speaking, do nothing
                            println!("Speaking");
                        },
                        false => {
                            // Active speech and not already speaking
                            speaking = true;
                            unactive_count = 0;
                        }
                    }
                },
                false => {
                    match speaking {
                        true => {
                            // Voice is not active and has been speaking
                            if unactive_count > SPEECH_BUFFER_COUNT {
                                /* 
                                    If more than 20 frames of unactive speech
                                    then consider end of segment and 
                                    transcribe
                                */ 
                                speaking = false;
                                // TODO process audio and get transcription
                            } else {
                                unactive_count += 1;
                            }
                        },
                        false => {
                            // Voice is not active and we are not speaking
                            // Do nothing
                            println!("Not speaking.");
                        }
                    }
                }
            }
        }
    }

    Ok(audio_data)
}