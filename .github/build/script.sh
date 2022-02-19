#!/bin/bash
set -x
export TIRA_SHA=${TIRA_SHA::8}
export EPOCH=$(date +%s)
JSON=$(cat .github/build/request.json | envsubst)
export JOBNAME=$(curl --fail -X POST -s -H 'Content-Type: application/json' -H "CF-Access-Client-Id: 46ea1be6ad585778416864f114d5cff1.access" -H "CF-Access-Client-Secret: ${ORACLE_K8S_ACCESS_TOKEN}" -d "${JSON}" https://kaniko.jrcichra.dev/kaniko | jq -r '.name')

CONTINUE=true
while "$CONTINUE";do
sleep 10
JSON=$(cat .github/build/progress.json | envsubst)
curl -s -X GET -H 'Content-Type: application/json' -H "CF-Access-Client-Id: 46ea1be6ad585778416864f114d5cff1.access" -H "CF-Access-Client-Secret: ${ORACLE_K8S_ACCESS_TOKEN}" https://kaniko.jrcichra.dev/kaniko  -d "${JSON}" | jq -r '.message' | grep -q 'completed successfully' && CONTINUE=false
done