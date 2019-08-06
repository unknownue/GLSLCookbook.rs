#!/bin/bash

if [ "$1" = "release" ]; then
    echo "Running examples in Release mode"
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

    cargo r --release --example chapter04 multi-light
    cargo r --release --example chapter04 directional
    cargo r --release --example chapter04 per-frag
    cargo r --release --example chapter04 spot
    cargo r --release --example chapter04 toon
    cargo r --release --example chapter04 fog
    cargo r --release --example chapter04 pbr

    cargo r --release --example chapter05 texture
    cargo r --release --example chapter05 multi-tex
    cargo r --release --example chapter05 nomrla-map
else
    echo "Running examples in Debug mode"
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

    cargo r --example chapter04 multi-light
    cargo r --example chapter04 directional
    cargo r --example chapter04 per-frag
    cargo r --example chapter04 spot
    cargo r --example chapter04 toon
    cargo r --example chapter04 fog
    cargo r --example chapter04 pbr

    cargo r --example chapter05 texture
    cargo r --example chapter05 multi-tex
    cargo r --example chapter05 normal-map
fi
