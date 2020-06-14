#!/bin/bash
set -euo pipefail

git log --pretty="%h %s" | grep part- | while read line; do
    HASH=$(echo $line | cut -d' ' -f1)
    BRANCH=$(echo $line | cut -d' ' -f2 | tr -d :)
    git branch --force $BRANCH $HASH
    git push origin $BRANCH --force
done

git branch --force part-0-end part-0.0
git push origin part-0-end --force

git branch --force part-1-end part-1.2
git push origin part-1-end --force

git branch --force part-2-end part-2.4
git push origin part-2-end --force

git branch --force part-3-end part-3.3
git push origin part-3-end --force

git branch --force part-4-end part-4.1
git push origin part-4-end --force

git branch --force part-5-end part-5.2
git push origin part-5-end --force
