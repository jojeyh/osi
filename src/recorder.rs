use std::i128::MIN;

use tokio::sync::mpsc::Sender;
use rtrb::{Consumer, RingBuffer};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleRate};
use webrtc_vad::{Vad, VadMode};

const BUFFER_FRAME_COUNT: usize = 30;
const MINIMUM_SAMPLE_COUNT: usize = 1600 * 4; // @ 16kHz = 400ms

#[derive(Debug)]
pub enum Command {
    AudioCommand {
        val: Vec<i16>,
    },
}

pub struct Recorder {}

impl Recorder {
    pub fn new() -> Self {
        Recorder {}
    }

    // TODO is this mutable self ok ?
    pub fn record(&self, tx: Sender<Command>) {
        // Initialize the voice activity detector 
        let mut vad = Vad::new_with_rate(webrtc_vad::SampleRate::Rate16kHz);
        vad.set_mode(VadMode::VeryAggressive);

        // Initialize and open mic stream
        let host = cpal::default_host();
        let device = host.default_input_device().expect("Failed to get default input device");
        let mut supported_configs_range = device.supported_input_configs()
            .expect("error while querying configs");
        let config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_sample_rate(SampleRate(16000))
            .config();
        println!("Using config: {:#?}", config);

        let (mut producer, mut consumer) = RingBuffer::<i16>::new(1024);
        let stream = device.build_input_stream(
            &config,
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

        // start the mic stream
        stream.play().expect("Stream play failed.");

        let mut unactive_count = 0;
        let mut speaking = false;
        let mut speech_segment = Vec::<i16>::new();
        /*
            TODO This is a dirty hack and should be changed to an algorithm
            that transcribes in short segments and also concatenates those segments 
            checking the results against one another, the choice of length of small vs 
            long segment will be hard to figure out
        */
        loop {
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
                                speech_segment.extend(audio_frame);
                            },
                            false => {
                                // Active speech and not already speaking
                                speaking = true;
                                unactive_count = 0;
                                speech_segment.extend(audio_frame);
                            }
                        }
                    },
                    false => {
                        match speaking {
                            true => {
                                // Voice is not active and has been speaking
                                if unactive_count > BUFFER_FRAME_COUNT {
                                    /* 
                                        If more than 20 frames of unactive speech
                                        then consider end of segment and 
                                        send over the channel to transcribing service
                                    */ 
                                    speaking = false;
                                    if speech_segment.len() > MINIMUM_SAMPLE_COUNT {
                                        let speech_segment_clone = speech_segment.clone();
                                        let tx_clone = tx.clone();
                                        tokio::spawn(async move {
                                            tx_clone.send(Command::AudioCommand { val: speech_segment_clone }).await.unwrap();
                                        });
                                    }
                                    speech_segment.clear();
                                } else {
                                    unactive_count += 1;
                                }
                            },
                            false => {
                                // Voice is not active and we are not speaking
                                // Do nothing
                            }
                        }
                    }
                }
            }
        }
    }
}