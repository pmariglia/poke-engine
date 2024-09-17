#!/usr/bin/env bash

pip install -r requirements.txt

rm -rf ./dist

maturin sdist -o ./dist

maturin upload ./dist/*.tar.gz
