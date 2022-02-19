#!/bin/bash
set -x
# do it for the sha
export TIRA_SHA="sha-${TIRA_SHA::8}"
docker manifest create ghcr.io/tjcichra/tira_backend:${TIRA_SHA} --amend \
    ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
docker manifest push ghcr.io/tjcichra/tira_backend:${TIRA_SHA}
# now for latest
export TIRA_SHA="latest"
docker manifest create ghcr.io/tjcichra/tira_backend:${TIRA_SHA} --amend \
    ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
docker manifest push ghcr.io/tjcichra/tira_backend:${TIRA_SHA}