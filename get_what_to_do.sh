#!/bin/bash

set -e
DIRS="./src/"

ADD_TAGS= #"IMPORTANT BUG"
results=$(grep -rn "TODO" $DIRS|grep -v binaire|grep -v "venv"|grep -v "data/"|grep -v "get_what_to_do.sh")

display_results(){
    echo "[*] Tagged as $1"|grep --color $1
    files=$(echo "$results"|grep $1|awk -F ':' '{print $1}'| tr ' ' '\n' | sort -u | tr '\n' ' ')
    #echo $files
    for f in $files
    do
        echo -e "\t"$f":"
        echo "$results"|grep $f|awk -F ':' '{$1=""; printf "\t\t%s -> ", $2; $2=""; gsub(/^[ \t]+|[ \t]+$/, ""); print $0}'|grep $1 --color
        echo
    done
    echo
}

for tag in $ADD_TAGS
do
    display_results "$tag"
done

if [ $# = '1' ]; then
    display_results "$1"
else
    display_results "TODO"
    display_results "IMPORTANT"
fi
