#!/usr/bin/env bash

# Print current tag and ask for new tag
current_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
echo "Current tag: $current_tag"
read -p "Enter the new tag: " new_tag

# Update CHANGELOG
git-cliff -c cliff.toml -u -t "$new_tag" -p CHANGELOG.md

# Update versions
sed -i -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$new_tag\"/" Cargo.toml
sed -i -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$new_tag\"/" poke-engine-py/Cargo.toml
sed -i -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$new_tag\"/" poke-engine-py/pyproject.toml

# Force Cargo.lock to update
cargo update -w

# Commit, tag, and push
git add Cargo* poke-engine-py/Cargo.toml poke-engine-py/pyproject.toml CHANGELOG.md
git commit -m "$new_tag"
git show
git tag -a "$new_tag" -m "$new_tag"

echo "Done! Tagged $new_tag"
echo "Verify the commit and push (git push origin main --tags)"
