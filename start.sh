#!/bin/bash
set -eu

if [[ -z $EDITOR ]]; then
	echo "EDITOR not set";
	exit 1;
fi

if fl=$(cargo run --bin start "$@"); then
	echo "Success?";
	echo "Opening $fl";
	$EDITOR "$fl"
else
	echo "not initiated: I'll refrain from opening it"
fi

