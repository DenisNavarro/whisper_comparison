
Comparing some Whisper implementations
======================================

**WARNING: this repository may become private in a near future.**

This project started as a technical test in an interview process. It aims to compare
the performances of two Whisper implementations: [whisper-burn][] and [whisper-rs][].

The requirements on Ubuntu include:

```bash
sudo apt install libclang-dev  # required to compile the whisper-rs crate
sudo apt install ffmpeg  # called by whisper_cpp_wrapper::ffmpeg_decoder::use_ffmpeg
```

[whisper-rs][] works while [whisper-burn][], with the last commit of 2023-08-07,
does not output what is expected.

To execute the code, launch a phony target from the [Makefile][].
It will download the needed files before the execution.

For example, `make bench` measures the performance of transcripting an audio file with
[whisper-rs][] with several models.

[Makefile]: ./Makefile
[whisper-burn]: https://github.com/Gadersd/whisper-burn
[whisper-rs]: https://github.com/tazz4843/whisper-rs
