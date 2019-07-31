#!/usr/bin/env bash

if [[ "$1" = "" ]] ; then
  echo "Error: needs to specify output location as argument"
  exit 1
elif [[ ! -d $1 ]] ; then
    echo "Error: $1 needs to an existing directory"
fi

pushd "$1" >/dev/null
OUTPUT_DIR=$(pwd)
popd >/dev/null

echo "output location is $OUTPUT_DIR"

cd src-data
for f in *
do
    if [[ -d "$f" ]] ; then
        pushd "$f" >/dev/null
        if [[ -e "generate.sh" ]] ; then
          ./generate.sh $OUTPUT_DIR
        fi
        popd >/dev/null
    fi
done