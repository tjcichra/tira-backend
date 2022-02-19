#!/bin/bash
set -x
export TIRA_SHA=${TIRA_SHA::8}
JSON=$(cat .github/build/request.json | envsubst)
curl --fail -X POST -s -H 'Content-Type: application/json' -H "CF-Access-Client-Id: 46ea1be6ad585778416864f114d5cff1.access" -H "CF-Access-Client-Secret: ${ORACLE_K8S_ACCESS_TOKEN}" -d "${JSON}" https://kaniko.jrcichra.dev/kaniko