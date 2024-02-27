#!/bin/bash

cd calculator
cargo build --target wasm32-unknown-unknown
cd ..
cp calculator/target/wasm32-unknown-unknown/debug/calculator.wasm client/calcplus.wasm

