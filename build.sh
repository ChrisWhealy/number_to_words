#!/usr/bin/env zsh

[ "$(cargo build)" -eq "0" ] && cp ./bin/debug/numbers-to-words ./bin
