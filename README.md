# Crowd Analysis using DDM

[![Build Status](https://travis-ci.com/jordanosborn/MastersProject.svg?token=2eJkyoJzDLeBMdiGDz2x&branch=master)](https://travis-ci.com/jordanosborn/MastersProject)
## Set Up
### Requirements
1. Ubuntu 18.04+ or macOS Mojave+
1. (Optional) NVIDIA Jetson TX2 - set up using Jetpack tool
1. OpenCV4
1. ArrayFire 3.6
1. Rust
1. Clang++
1. Python3.6+
1. [Poetry](https://github.com/sdispater/poetry)
#### Install on Ubuntu:
[Download](https://github.com/jordanosborn/CrowdAnalysisDDM/raw/master/install_ubuntu.sh) installer script.

    chmod +x install_ubuntu.sh
    ./install_ubuntu.sh
#### Install on Mac:
TODO - follow README in [code folder](code/README.md).
### Usage - Command Line Arguments

Inside code directory replace {arg} with path/ numerical value (positive integers)

#### Cargo
1. cargo run --release video-ddm {frame_buffer_capacity} {annuli_spacing} {video_path} {output_csv_path}
1. cargo run --release camera-ddm {frame_buffer_capacity} {outup_csv_path}
1. cargo run --release camera-multi-ddm {frame_buffer_capacity} {annuli_spacing} {tile_min_size} {tile_max_size} {number_tiles} {output_directory_path}
1. cargo run --release video-multi-ddm {frame_buffer_capacity} {annuli_spacing} {tile_min_size} {tile_max_size} {number_tiles} {video_path} {output_directory_path}

#### Python automation script
TODO
