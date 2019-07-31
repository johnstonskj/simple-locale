#!/usr/bin/env bash

if [[ "$1" = "" ]] ; then
  echo "Error: needs to specify output location as argument"
  exit 1
elif [[ ! -d $1 ]] ; then
    echo "Error: $1 needs to an existing directory"
fi

echo "generating $(basename $(pwd)) files into $1"
python generate.py >> $1/codes/languages.rs