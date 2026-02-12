#!/bin/sh

printf 'P2
2 2
255
1 2
3 4
' |
    wasmtime \
        run \
        ./rs-pnm-shrink.wasm \
        --size-hint tiny \
        --filter catmull-rom \
        --aspect preserve |
        file -
