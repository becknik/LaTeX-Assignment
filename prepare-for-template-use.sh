#!/bin/bash

read -p "Remove the git specific stuff from template directory? [y/n]" remove_git_files

if [ ${remove_git_files} = "y" ] ; then

    rm --recursive --force ./.git
    rm --force ./README.md
else

    echo "Aborted."
fi