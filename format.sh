#!/usr/bin/env bash

## format all rs files in cluster-rs
for file in $(find cluster-rs -name '*.rs'); do
    rustfmt $file
done

echo "Format complete."