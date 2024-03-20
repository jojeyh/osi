use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};

pub struct Transcriber {
    ctx: WhisperContext,
}

impl Transcriber {
    pub fn new() -> Self {
        /* 
            TODO remove hardcoded model path and handle command line or default
            while also downloading if no model found
        */
        let ctx = WhisperContext::new_with_params(
            "models/ggml-large-v3-q5_0.bin",
            WhisperContextParameters::default()
        ).expect("Failed to load model.");


        Transcriber {
            ctx
        }
    }

    pub fn transcribe(&self, audio_data: Vec<f32>) {
        // TODO can this be refactored to using new instead of f
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(8);
        params.set_translate(false);
        params.set_language(Some("es"));

        let mut state = self.ctx.create_state()
            .expect("Failed to create state.");
        state
            .full(params, &audio_data[..])
            .expect("failed to run model");

        // fetch the results
        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .expect("failed to get segment");
            let start_timestamp = state
                .full_get_segment_t0(i)
                .expect("failed to get segment start timestamp");
            let end_timestamp = state
                .full_get_segment_t1(i)
                .expect("failed to get segment end timestamp");
            println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        }
    }
}