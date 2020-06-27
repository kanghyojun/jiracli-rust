#!/usr/bin/env bash

if [[ "$(git config core.hooksPath)" = "" ]];
then
    git config core.hooksPath "$(git rev-parse --show-toplevel)/hooks"
fi

cargo build
