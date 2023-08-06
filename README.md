
Comparing some Whisper implementations
======================================

This project started as a technical test to compare the performances of two Whisper
implementations: [whisper-burn][] and [whisper-rs][].

The requirements on Ubuntu include:

```bash
sudo apt install libclang-dev  # required to compile the whisper-rs crate
sudo apt install ffmpeg  # called by whisper_cpp_wrapper::ffmpeg_decoder::use_ffmpeg
```

[whisper-burn][] panicked while [whisper-rs][] worked, so the comparison was simpler than expected.

To execute the code, launch a phony target from the [Makefile][].
It will download the needed files before the execution.

For example, `make bench` measures the performance of transcripting an audio file with
[whisper-rs][] with several models.

Note that most of the code of this repository was adapted from:

  + <https://github.com/Gadersd/whisper-burn/tree/3757c15fd18fe2ec2c398cb6a4697e108442ff3a>
  + <https://github.com/m1guelpf/whisper-cli-rs/tree/v0.1.4>

So most of the code from this repository does not show how I code from scratch,
contrary to the [bin_from_ninja][] POC.

[bin_from_ninja]: https://github.com/DenisNavarro/rust_pocs/tree/main/bin_from_ninja
[Makefile]: ./Makefile
[whisper-burn]: https://github.com/Gadersd/whisper-burn
[whisper-rs]: https://github.com/tazz4843/whisper-rs
