#!/bin/bash
set -x

# env vars
export TIRA_SHA="sha-${TIRA_SHA::8}"
# pull the containers
docker pull ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA}
docker pull ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}

# update the current sha on the base image
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

# if main branch, update the latest tag on the base image
if [ "$BRANCH_NAME" == "main" ] || [ "$BRANCH_NAME" == "master" ]; then
    docker manifest create ghcr.io/tjcichra/tira_backend:latest \
        --amend ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
        --amend ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA}
    docker manifest annotate ghcr.io/tjcichra/tira_backend:latest \
        ghcr.io/tjcichra/tira_backend_amd64:${TIRA_SHA} \
        --os linux \
        --arch amd64
    docker manifest annotate ghcr.io/tjcichra/tira_backend:latest \
        ghcr.io/tjcichra/tira_backend_arm64:${TIRA_SHA} \
        --os linux \
        --arch arm64
    docker manifest push ghcr.io/tjcichra/tira_backend:latest
fi