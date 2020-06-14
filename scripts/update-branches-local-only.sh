#!/usr/bin/env bash
set -euo pipefail

git log --pretty="%h %s" | grep part- | while read line; do
    HASH=$(echo $line | cut -d' ' -f1)
    BRANCH=$(echo $line | cut -d' ' -f2 | tr -d :)
    git branch --force $BRANCH $HASH
done

