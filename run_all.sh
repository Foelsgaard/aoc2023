#!/bin/sh

INDIR=${1:-input}

for i in $(seq -f "%02g" 1 25)
do
    test -f $INDIR/day$i && ./target/release/day$i $INDIR/day$i
done
