#!/bin/bash

# Make a tmp folder and change to it
TEST_GIT_TMP_DIR=$(mktemp -d)
cd $TEST_GIT_TMP_DIR

# Create new git repo
git init

# Create some files
echo "Working Tree New" > working_tree_new
echo "Working Tree Modified" > working_tree_modified
echo "Working Tree Deleted" > working_tree_deleted

echo "Staged New" > staged_new
echo "Staged Modified" > staged_modified
echo "Staged Deleted" > staged_deleted
echo "Staged Renamed" > staged_renamed

# Do some things with git so we can have a bunch of different statuses at once

# Add and commit a bunch of files so we can do things with them later
git add working_tree_modified working_tree_deleted staged_modified staged_deleted staged_renamed
git commit --no-gpg-sign -m "Setup commit"

# Make changes to the working tree
echo "This is a change" >> working_tree_modified

rm working_tree_deleted

# Make changes to the stageing index
git add staged_new

echo "This is a change" >> staged_modified
git add staged_modified

git rm staged_deleted

git mv staged_renamed staged_renamed_new_name


echo "Created test repo at $TEST_GIT_TMP_DIR"
