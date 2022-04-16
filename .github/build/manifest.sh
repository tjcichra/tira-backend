#!/bin/bash
set -x

# env vars
export TIRA_SHA="sha-${TIRA_SHA::7}"
# pull the containers
docker pull ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-amd64
docker pull ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-arm64

#https://www.docker.com/blog/multi-arch-build-and-images-the-simple-way/

# update the current sha on the base image
docker manifest create ghcr.io/tjcichra/tira_backend:${TIRA_SHA} \
    --amend ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-amd64 \
    --amend ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-arm64
docker manifest push ghcr.io/tjcichra/tira_backend:${TIRA_SHA}

# if main branch, update the latest tag on the base image
if [ "$BRANCH_NAME" == "main" ] || [ "$BRANCH_NAME" == "master" ]; then
    docker manifest create ghcr.io/tjcichra/tira_backend:latest \
        --amend ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-amd64 \
        --amend ghcr.io/tjcichra/tira_backend:${TIRA_SHA}-arm64
    docker manifest push ghcr.io/tjcichra/tira_backend:latest
fi