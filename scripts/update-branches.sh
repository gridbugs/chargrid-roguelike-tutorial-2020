#!/usr/bin/env bash
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

git branch --force part-6-end part-6.5
git push origin part-6-end --force

git branch --force part-7-end part-7.2
git push origin part-7-end --force

git branch --force part-8-end part-8.5
git push origin part-8-end --force

git branch --force part-9-end part-9.3
git push origin part-9-end --force

git branch --force part-10-end part-10.3
git push origin part-10-end --force

git branch --force part-11-end part-11.4
git push origin part-11-end --force

git branch --force part-12-end part-12.0
git push origin part-12-end --force

git branch --force part-13-end part-13.4
git push origin part-13-end --force
