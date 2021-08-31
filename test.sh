#!/usr/bin/env bash

# prevent sourcing of this script, only allow execution
$(return >/dev/null 2>&1)
test "$?" -eq "0" && { echo "This script should only be executed." >&2; exit 1; }

# exit on errors, undefined variables, ensure errors in pipes are not hidden
set -Eeuo pipefail

# set log id and use shared log function for readable logs
declare mydir
mydir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

# helper functions
log() {
  local time
  # second-precision is enough
  time=$(date -u +%y-%m-%dT%H:%M:%SZ)
  echo >&2 -e "${time} [test] ${1-}"
}

msg() {
  echo >&2 -e "${1-}"
}

# work

declare dev_port="32132"

# will be set to 1 by any failing test
declare test_failed=0

function cleanup {
  local EXIT_CODE=$?

  # at this point we don't want to fail hard anymore
  trap - SIGINT SIGTERM ERR EXIT
  set +Eeuo pipefail

  log "shut down wrangler-dev if still running"
  lsof -i ":${dev_port}" -s TCP:LISTEN -t | xargs -I {} -n 1 kill {}

  exit $EXIT_CODE
}

trap cleanup SIGINT SIGTERM ERR EXIT

function curl_call() {
  declare path="${1}"
  declare method="${2:-POST}"

  curl -X ${method} -s -o /dev/null -w '%{http_code}' \
    -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":1}' \
    http://127.0.0.1:${dev_port}/${path}
}

log "check if wrangler-dev is running on port ${dev_port}"
if ! nc -z -w 1 127.0.0.1 ${dev_port}; then
  log "start wrangler-dev on port ${dev_port}"
  yarn wrangler dev -p ${dev_port} &
  sleep 10
fi

declare methods="HEAD OPTIONS TRACE PUT PATCH GET DELETE"
for method in ${methods}; do
  log "test unsupported HTTP method - ${method}"
  if [ ! "$(curl_call "anypath" "${method}")" -eq "405" ]; then
    log "test unsupported HTTP method - ${method} - FAILED"
    test_failed=1
  else
    log "test unsupported HTTP method - ${method} - OK"
  fi
done

log "test unconfigured path - /anypath"
if [ ! "$(curl_call "anypath")" -eq "404" ]; then
  log "test unconfigured path - /anypath - FAILED"
  test_failed=1
else
  log "test unconfigured path - /anypath - OK"
fi

declare providers=$(grep -E "(eth_|xdai_|matic_)" src/providers.ts | awk -F: "{ print \$1; }" | tr -d ' ')
for provider in ${providers}; do
  log "test supported provider - ${provider}"
  if [ ! "$(curl_call "${provider}")" -eq "200" ]; then
    log "test supported provider - ${provider} - FAILED"
    test_failed=1
  else
    log "test supported provider - ${provider} - OK"
  fi
done

exit ${test_failed}
