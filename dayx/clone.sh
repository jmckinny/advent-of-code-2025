#!/usr/bin/env bash

set -e

DAY_NUM=$1

if [ -z "$DAY_NUM" ]; then
  echo "Usage: clone.sh <day_number>"
  exit -1
fi

cd "$(dirname "$0")"
cd ..

FILE_PREFIX="day$DAY_NUM"
mkdir "$FILE_PREFIX"
cp dayx/dayx.go "$FILE_PREFIX/$FILE_PREFIX.go"
cp dayx/dayx_test.go "$FILE_PREFIX/${FILE_PREFIX}_test.go"
cp dayx/go.mod "$FILE_PREFIX/go.mod"
sed -i "s/dayx/$FILE_PREFIX/g" "$FILE_PREFIX/go.mod"
