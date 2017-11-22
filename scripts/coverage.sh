#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

for file in target/debug/distribution-*[^\.d]; do
    mkdir -p "target/cov/$(basename $file)"
    kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done
