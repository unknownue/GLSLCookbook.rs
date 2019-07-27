#!/bin/bash

if [ "$1" = "release" ]; then
    echo "Running example in Release mode"
    cargo r --release --example chapter01 basic

    cargo r --release --example chapter02 basic-attrib
    cargo r --release --example chapter02 basic-uniform
    cargo r --release --example chapter02 basic-uniform-block

    cargo r --release --example chapter03 diffuse
    cargo r --release --example chapter03 phong
    cargo r --release --example chapter03 two-side
    cargo r --release --example chapter03 flat
    cargo r --release --example chapter03 subroutine
    cargo r --release --example chapter03 discard

    cargo r --release --example chapter04 directional
else
    echo "Running example in Debug mode"
    cargo r --example chapter01 basic

    cargo r --example chapter02 basic-attrib
    cargo r --example chapter02 basic-uniform
    cargo r --example chapter02 basic-uniform-block

    cargo r --example chapter03 diffuse
    cargo r --example chapter03 phong
    cargo r --example chapter03 two-side
    cargo r --example chapter03 flat
    cargo r --example chapter03 subroutine
    cargo r --example chapter03 discard

    cargo r --example chapter04 directional
fi