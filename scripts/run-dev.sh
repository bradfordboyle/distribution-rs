#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

docker run \
    --privileged \
    --tty \
    --interactive \
    --rm=true \
    --volume "${DIST_HOME}":/opt/distribution \
    distribution-dev \
    /bin/bash
