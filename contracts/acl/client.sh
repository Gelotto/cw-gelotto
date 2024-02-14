#!/bin/bash

CMD=$1
NETWORK=$2
NODE=
CHAIN_ID=
FLAGS=

TAG=$3
if [ -z "$TAG" ]; then
  TAG=$(cat ./builds/latest)
fi

CONTRACT_ADDR=$(cat ./builds/build-$TAG/latest-contract)

shift 3

case $NETWORK in
  testnet)
    NODE="https://rpc.uni.juno.deuslabs.fi:443"
    CHAIN_ID=uni-3
    DENOM=ujunox
    ;;
  mainnet)
    NODE="https://rpc-juno.itastakers.com:443"
    CHAIN_ID=juno-1
    DENOM=ujuno
    ;;
  devnet)
    NODE="http://localhost:26657"
    CHAIN_ID=testing
    DENOM=ujunox
    ;;
esac


allow() {
  sender=$1
  principal=$2
  resource=$3
  msg='{"principal":{"allow":{"principal":{"address":"'$principal'"},"resources":["'$resource'"]}}}'
  flags="\
  --node $NODE \
  --gas-prices 0.09$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.3 \
  -b sync \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "'"$msg"'" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" $msg $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

deny() {
  sender=$1
  principal=$2
  resource=$3
  msg='{"principal":{"deny":{"principal":{"address":"'$principal'"},"resources":["'$resource'"]}}}'
  flags="\
  --node $NODE \
  --gas-prices 0.09$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.3 \
  -b sync \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "'"$msg"'" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" $msg $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}


allow-role() {
  sender=$1
  role=$2
  resource=$3
  msg='{"principal":{"allow":{"principal":{"role":"'$role'"},"resources":["'$resource'"]}}}'
  flags="\
  --node $NODE \
  --gas-prices 0.08$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.3 \
  -b sync\
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "'"$msg"'" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" $msg $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

grant-role() {
  sender=$1
  principal=$2
  role=$3
  msg='{"principal":{"grant_role":{"principal":{"address":"'$principal'"},"roles":["'$role'"]}}}'
  flags="\
  --node $NODE \
  --gas-prices 0.09$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.3 \
  -b sync \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "'"$msg"'" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" $msg $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

is_authorized() {
  principal=$2
  action=$3
  query='{"principal":{"is_allowed": {"principal":{"address":"'$principal'"},"resources":["'$action'"]}}}'
  flags="--chain-id $CHAIN_ID --output json --node $NODE"
  echo junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags
  response=$(junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

set -e

echo $*
case $CMD in
  allow)
    allow $1 $2 $3
    ;;
  deny)
    deny $1 $2 $3
    ;;
  allow-role)
    allow-role $1 $2 $3
    ;;
  grant-role)
    grant-role $1 $2 $3
    ;;
  is-authorized) 
    is_authorized $1 $2 $3
    ;;
esac