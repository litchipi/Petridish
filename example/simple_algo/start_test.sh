#!/bin/bash

if [ ! -f target/release/libsimple_algo.so ] ; then
    echo -e "Must build lib before:\n\tcargo build --release"
fi

if [ ! -e ./genalgo.so ] ; then
    ln -s target/release/libsimple_algo.so ./genalgo.so
fi
python3 ./unitest_py_iface.py
