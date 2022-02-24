#!/bin/bash
# Copyright 2021 The BeanCloudServices Developers.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# This is just a little script that can be downloaded from the internet to download and install cloud-contracts
# and runs it. Some functions are copied from https://github.com/rustwasm/wasm-pack/blob/master/docs/_installer/init.sh

env=dev
gh_pat_path="$HOME/.beancloud/.gh_pat"
organisation=BeanCloudServices
contract_repo=cloud-contracts
app_dir=cdn-api-app

set -u

main() {
    get_machine

    contract_type="$1"    
    bounded_context="$2"    
    entity_name="$3"
    version="$4"

    need_gh_pat

    need_cmd mktemp
    need_cmd chmod
    need_cmd mkdir
    need_cmd rm
    need_cmd rmdir
    need_cmd tar
    need_cmd which
    need_cmd dirname
    need_cmd uname

    need_cmd java "https://sdkman.io/install"
    need_cmd jq "Please make sure you have jq installed: https://stedolan.github.io/jq/download/"
    need_cmd curl

    which rustup > /dev/null 2>&1
    need_ok "failed to find Rust installation, is rustup installed?"
    local _rustup=`which rustup`

    clean
    local _dir="./contracts/$bounded_context/openapi"
    # ensure mkdir -p $_dir

    printf '%s\n' "info: downloading $bounded_context $entity_name $contract_type contract" 1>&2

    download_contract
    unpack_contract

    local _retval=$?
    return "$_retval"
}

get_machine() {
    unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)     machine=Linux;;
    Darwin*)    machine=Mac;;
    CYGWIN*)    machine=Cygwin;;
    MINGW*)     machine=MinGw;;
    *)          machine="UNKNOWN:${unameOut}"
esac
}

need_gh_pat() {
    if [ -f "$gh_pat_path" ]
    then
        export gh_pat=`cat $gh_pat_path`
    else
        say "Please create a Github Personal Access Token https://github.com/settings/tokens/new with full repo permissions \n \n"
        read_input_or "If you already have a token, please enter here" "empty"
        if [ $read_input == "empty"  ]; then
          printf "Empty input. Exiting... \n"
          exit
        fi
        
        mkdir -p ~/.beancloud
        
        export gh_pat=$read_input
        echo $read_input > $gh_pat_path
    fi

}

read_input_or () {
    
    printf -- "$1\n"
    
    read read_input
    
    if [ -z "$read_input" ]; then
        read_input=$2
    fi
}

clean() {
   	rm -fr ./contracts
	rm -fr *-openapi
    rm -f _contracts.tar.gz
}

say() {
    printf "unpack: $1"
}

err() {
    say "$1" >&2
    local _custom_msg=${2-undefined}
    if [ "undefined" != "$_custom_msg" ]; then
        say "$_custom_msg"
    fi
    exit 1
}

need_cmd() {
    local _custom_msg=${2-undefined}
    if ! check_cmd "$1"
    then 
        err "need '$1' (command not found)" "$_custom_msg"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

need_ok() {
    if [ $? != 0 ]; then err "$1"; fi
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
    "$@"
    need_ok "command failed: $*"
}

# This is just for indicating that commands' results are being
# intentionally ignored. Usually, because it's being executed
# as part of error handling.
ignore() {
    "$@"
}

download_contract() {
    curl -s \
    --location \
    -H "Authorization: token $gh_pat" \
  -X GET \
  -H "Accept: application/vnd.github.v3+json" \
https://api.github.com/repos/$organisation/$contract_repo/tarball > _contracts.tar.gz
tar xvf _contracts.tar.gz
local _contract_dir=`find . -name ${organisation}-${contract_repo}*`
mv -f $_contract_dir/contracts ./contracts
rm -f -r $_contract_dir
}

unpack_contract() {
    echo "Unpacking ... [$env-$entity_name]"
    local _dir_entity="../../../$app_dir/$entity_name-openapi"
    rm -f -r $_dir_entity
	cp -R ./bin/resources/openapi/custom_templates/* ./bin/resources/openapi/rust-server/

    cd ./contracts/$bounded_context/openapi
	java -jar ../../../bin/resources/openapi-generator-cli.jar generate -i ./$entity_name.yaml -o $_dir_entity --package-name=beancloud_${bounded_context}_openapi_$entity_name -g rust-server -t ../../../bin/resources/openapi/rust-server -c ../../../bin/resources/openapi/config.yaml --type-mappings=date=NaiveDate

	# java -jar ../../../bin/resources/openapi-generator-cli.jar generate -i ./$entity_name.yaml -o $_dir_entity --package-name=beancloud_${bounded_context}_openapi_$entity_name -g rust-server --type-mappings=date=NaiveDate

	# rm -fr $_dir_entity/examples
	# rm -fr $_dir_entity/rust-server
	# rm -fr $_dir_entity/src/client
	# rm -fr $_dir_entity/src/server
	# rm $_dir_entity/src/context.rs
	# rm $_dir_entity/src/header.rs

	cd $_dir_entity &&  cargo fmt --all
    cd ../../../
}

main "$@" || exit 1