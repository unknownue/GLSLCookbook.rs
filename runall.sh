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
    cargo r --release --example chapter05 parallax
    cargo r --release --example chapter05 reflect-cube
    cargo r --release --example chapter05 refract-cube
    cargo r --release --example chapter05 proj-tex
    cargo r --release --example chapter05 render-to-tex
    cargo r --release --example chapter05 sampler-obj
    cargo r --release --example chapter05 diff-ibl

    cargo r --release --example chapter06 edge
    cargo r --release --example chapter06 blur
    cargo r --release --example chapter06 tone-map
    cargo r --release --example chapter06 hdr-bloom
    cargo r --release --example chapter06 gamma
    cargo r --release --example chapter06 msaa
    cargo r --release --example chapter06 deferred
    cargo r --release --example chapter06 ssao

    cargo r --release --example chapter07 point-sprite
    cargo r --release --example chapter07 shade-wire
    cargo r --release --example chapter07 silhouette
    cargo r --release --example chapter07 bez-curve
    cargo r --release --example chapter07 quad-tess
    cargo r --release --example chapter07 tess-teapot
    cargo r --release --example chapter07 tess-teapot-depth

    cargo r --release --example chapter08 shadow-map
    cargo r --release --example chapter08 pcf
    cargo r --release --example chapter08 ao

    cargo r --release --example chapter09 noise
    cargo r --release --example chapter09 sky
    cargo r --release --example chapter09 wood
    cargo r --release --example chapter09 decay
    cargo r --release --example chapter09 paint
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
    cargo r --example chapter05 parallax
    cargo r --example chapter05 reflect-cube
    cargo r --example chapter05 refract-cube
    cargo r --example chapter05 proj-tex
    cargo r --example chapter05 render-to-tex
    cargo r --example chapter05 sampler-obj
    cargo r --example chapter05 diff-ibl

    cargo r --example chapter06 edge
    cargo r --example chapter06 blur
    cargo r --example chapter06 tone-map
    cargo r --example chapter06 hdr-bloom
    cargo r --example chapter06 gamma
    cargo r --example chapter06 msaa
    cargo r --example chapter06 deferred
    cargo r --example chapter06 ssao

    cargo r --example chapter07 point-sprite
    cargo r --example chapter07 shade-wire
    cargo r --example chapter07 silhouette
    cargo r --example chapter07 bez-curve
    cargo r --example chapter07 quad-tess
    cargo r --example chapter07 tess-teapot
    cargo r --example chapter07 tess-teapot-depth

    cargo r --example chapter08 shadow-map
    cargo r --example chapter08 pcf
    cargo r --example chapter08 ao

    cargo r --example chapter09 noise
    cargo r --example chapter09 sky
    cargo r --example chapter09 wood
    cargo r --example chapter09 decay
    cargo r --example chapter09 paint
fi
