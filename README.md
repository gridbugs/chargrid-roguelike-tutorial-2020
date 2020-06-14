# Chargrid Roguelike Tutorial 2020

[![dependency status](https://deps.rs/repo/github/stevebob/chargrid-roguelike-tutorial-2020/status.svg)](https://deps.rs/repo/github/stevebob/chargrid-roguelike-tutorial-2020)
[![test](https://github.com/stevebob/chargrid-roguelike-tutorial-2020/actions/workflows/test.yml/badge.svg)](https://github.com/stevebob/chargrid-roguelike-tutorial-2020/actions/workflows/test.yml)

Code for [this tutorial](https://gridbugs.org/roguelike-tutorial-2020/).

## Editing

The commit history in this repo is intended to follow the sections in the
tutorial. When updating the code in this repository, care must be taken to
preserve the correspondence between this history and the tutorial.
To edit this repo, perform an interactive rebase from the beginning of time:
```
git rebase -i --root
```

Edit the relevant commits, then run the `scripts/update-branches.sh` script to
move all the branch pointers to their post-rebase counterparts (based on labels
in commit messages), then force push each branch to github.

Don't forget to also force push the main branch to be consistent with the
end of the tutorial.
