# Purpose

This repository is intended as a base to showcase how git can tell you a story,
and what it may look like if you were not to manage your commit history cleanly.

The primary branch is the `main` branch, but `squashed` will showcase what
this repository will look like if you were to squash down commits such as
in-progress commits before a code review is submitted or those that are meant to
fixup one of the earlier commits while multiple are in a code review state,
but are inter-dependent. The in-progress commits are marked with `partial:`,
while the changes that are fixed with `git rebase --i -autosquash` have the typical
`fixup!` prefix provided by the `git` CLI itself.

A bad commit history is provided in the `garbage` branch. Note that the commit
messages are hashed to exaggerate the effect of a commit history where care is not taken.

Test cases are added within each commit, but this repository can also be used to demostrate
using git-provided tools such as `git bisect`. The `garbage-bugged` branch would be hard to
debug which commit caused the issue without the help of automated tools due to the volume of
commits. Try manually debugging this branch, and then try again using the follow `git bisect`
commands.

```sh
git bisect start
git bisect bad
librs_first_commit=$(git log --pretty=format:"%h" --diff-filter=A -- src/lib.rs)
git bisect good $librs_first_commit
git bisect run sh -c 'cargo run -- 9007199254740993 1 | grep -q "^9007199254740994$" || exit 1'
```

The history was mangled using
```sh
git filter-branch -f --msg-filter 'uuidgen' HEAD
```
