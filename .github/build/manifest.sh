#!/bin/bash
set -x
# do it for the sha
export TIRA_SHA="sha-${TIRA_SHA::8}"
docker manifest create ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
docker manifest push ghcr.io/tjcichra/tira_backend:${TIRA_SHA}

# set the current sha to the latest
docker manifest create ghcr.io/tjcichra/tira_backend:latest \
    --amend ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
docker manifest push ghcr.io/tjcichra/tira_backend:latest
