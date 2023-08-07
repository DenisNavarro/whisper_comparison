use criterion::{criterion_group, criterion_main, Criterion};

use whisper_comparison::whisper_cpp_wrapper::model::Size;
use whisper_comparison::whisper_cpp_wrapper::AudioTranscripter;

fn bench_whisper_cpp_wrapper(c: &mut Criterion) {
    let lang = None;
    let translate = false;
    let karaoke = false;
    let audio_file_path = "whisper_cpp_data/audio.wav";
    let mut group = c.benchmark_group("whisper_cpp_wrapper");
    for (id, model) in [("tiny", Size::Tiny), ("base", Size::Base), ("small", Size::Small)] {
        let mut audio_transcripter = None;
        group.bench_function(id, |b| {
            let audio_transcripter = audio_transcripter.get_or_insert_with(|| {
                AudioTranscripter::new(model, lang, translate, karaoke).unwrap()
            });
            b.iter(|| audio_transcripter.transcript(audio_file_path))
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_whisper_cpp_wrapper
);

criterion_main!(benches);
