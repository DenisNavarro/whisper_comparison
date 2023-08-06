use criterion::{criterion_group, criterion_main, Criterion};

use whisper_comparison::whisper_cpp_wrapper::model::Size;
use whisper_comparison::whisper_cpp_wrapper::AudioTranscripter;

pub fn tiny_benchmark(c: &mut Criterion) {
    benchmark(c, "tiny", Size::Tiny);
}

pub fn base_benchmark(c: &mut Criterion) {
    benchmark(c, "base", Size::Base);
}

pub fn small_benchmark(c: &mut Criterion) {
    benchmark(c, "small", Size::Small);
}

fn benchmark(c: &mut Criterion, id: &str, model: Size) {
    let lang = None;
    let translate = false;
    let karaoke = false;
    let audio_file_path = "whisper_cpp_data/audio.wav";
    let mut audio_transcripter = AudioTranscripter::new(model, lang, translate, karaoke).unwrap();
    c.bench_function(id, |b| b.iter(|| audio_transcripter.transcript(audio_file_path)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = tiny_benchmark, base_benchmark, small_benchmark
);

criterion_main!(benches);
