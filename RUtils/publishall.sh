#!/bin/bash

for dir in */
do
    if [ -d $dir ]
    then
        pushd $dir >/dev/null
        if [ -f publish.sh ]
        then
            echo "-----------------------------------------"
            pwd
            ./publish.sh
        fi
        popd >/dev/null
    fi
done
