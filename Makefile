
.DELETE_ON_ERROR:
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --warn-undefined-variables

debug_exe_path = target/debug/whisper_comparison
release_exe_path = target/release/whisper_comparison

.PHONY: debug_help
#: Debug execution with help
debug_help : $(debug_exe_path) clippy.maketarget
	$< help

#############################################
# Other phony targets in alphabetical order #
#############################################

.PHONY: bench
#: Launch benchmark
bench : clippy.maketarget whisper_cpp_data
	cargo bench

.PHONY: clean
#: Remove what is in .gitignore
clean :
	git clean -dXf

.PHONY: debug_burn
#: Debug execution with whisper-burn and crash
debug_burn : $(debug_exe_path) clippy.maketarget whisper_burn_data
	$< burn

.PHONY: debug_cpp_base
#: Debug execution with whisper-rs (Rust bindings to whisper.cpp) with the base model
debug_cpp_base : $(debug_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp base
	cat whisper_cpp_data/audio.wav.txt

.PHONY: debug_cpp_small
#: Debug execution with whisper-rs (Rust bindings to whisper.cpp) with the small model
debug_cpp_small : $(debug_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp small
	cat whisper_cpp_data/audio.wav.txt

.PHONY: debug_cpp_tiny
#: Debug execution with whisper-rs (Rust bindings to whisper.cpp) with the tiny model
debug_cpp_tiny : $(debug_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp tiny
	cat whisper_cpp_data/audio.wav.txt

.PHONY: download
#: Download files
download : whisper_burn_data whisper_cpp_data

.PHONY: help
#: Print the help with remake from https://remake.readthedocs.io/
help :
	@remake --tasks

.PHONY: release_burn
#: Release execution with whisper-burn and crash
release_burn : $(release_exe_path) clippy.maketarget whisper_burn_data
	$< burn

.PHONY: release_cpp_base
#: Release execution with whisper-rs (Rust bindings to whisper.cpp) with the base model
release_cpp_base : $(release_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp base
	cat whisper_cpp_data/audio.wav.txt

.PHONY: release_cpp_small
#: Release execution with whisper-rs (Rust bindings to whisper.cpp) with the small model
release_cpp_small : $(release_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp small
	cat whisper_cpp_data/audio.wav.txt

.PHONY: release_cpp_tiny
#: Release execution with whisper-rs (Rust bindings to whisper.cpp) with the tiny model
release_cpp_tiny : $(release_exe_path) clippy.maketarget whisper_cpp_data
	$< cpp tiny
	cat whisper_cpp_data/audio.wav.txt

.PHONY: release_help
#: Release execution with help
release_help : $(release_exe_path) clippy.maketarget
	$< help

##############################
# File and directory targets #
##############################

#: Check the code with Clippy
clippy.maketarget : fmt.maketarget
	cargo clippy -- -D warnings && touch $@

#: Reformat the Rust files
fmt.maketarget : rustfmt.toml $(wildcard src/*.rs) $(wildcard src/**/*.rs)
	cargo fmt && touch $@

$(debug_exe_path) : Cargo.toml fmt.maketarget
	cargo build

$(release_exe_path) : Cargo.toml fmt.maketarget
	cargo build --release

whisper_burn_data :
	mkdir burn_tmp
	wget -O burn_tmp/tiny.cfg https://huggingface.co/Gadersd/whisper-burn/resolve/main/tiny/tiny.cfg
	wget -O burn_tmp/tiny.mpk.gz https://huggingface.co/Gadersd/whisper-burn/resolve/main/tiny/tiny.mpk.gz
	wget -O burn_tmp/audio.wav https://github.com/Gadersd/whisper-burn/raw/3757c15fd18fe2ec2c398cb6a4697e108442ff3a/audio.wav
	mv burn_tmp $@

whisper_cpp_data :
	mkdir cpp_tmp
	wget -O cpp_tmp/tiny.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin
	wget -O cpp_tmp/base.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
	wget -O cpp_tmp/small.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin
	wget -O cpp_tmp/audio.wav https://github.com/Gadersd/whisper-burn/raw/3757c15fd18fe2ec2c398cb6a4697e108442ff3a/audio.wav
	mv cpp_tmp $@
