#!/usr/bin/env bash

echo "generating $(basename $(pwd)) files into $1"
python generate.py > $1/codes/languages-data.rs