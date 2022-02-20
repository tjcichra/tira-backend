#!/bin/bash
set -x
# do it for the sha
export TIRA_SHA="sha-${TIRA_SHA::8}"
docker manifest create ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
docker manifest annotate ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
    ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
    --os linux \
    --arch amd64
docker manifest annotate ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
    ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA} \
    --os linux \
    --arch arm64
docker manifest push ghcr.io/tjcichra/tira_backend:${TIRA_SHA}

# set the current sha to the latest if we're on the default branch
if [ "$DEFAULT_BRANCH" == "main" ] || [ "$DEFAULT_BRANCH" == "master" ]; then
    docker manifest create ghcr.io/tjcichra/tira_backend:latest \
        --amend ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
        --amend ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
    docker manifest annotate ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
        ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
        --os linux \
        --arch amd64
    docker manifest annotate ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
        ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA} \
        --os linux \
        --arch arm64
    docker manifest push ghcr.io/tjcichra/tira_backend:latest
fi