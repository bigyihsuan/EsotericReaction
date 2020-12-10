#!/bin/bash

mcs -out:./EsoReaction src/*.cs && mono EsoReaction $1 $2 $3