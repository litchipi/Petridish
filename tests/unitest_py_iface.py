#!/usr/bin/env python3
#-*-encoding:utf-8*-

import genalgo

def test0():
    genalgo.algo_test.create_algo_test(genalgo.get_lab_default()).start(100)

ALL_TESTS = [
        test0
        ]

if __name__ == "__main__":
    print("Starting unitests")
    for n, test in enumerate(ALL_TESTS):
        try:
            print("\nTest {}/{}: ".format(n+1, len(ALL_TESTS)))
            test()
        except Exception as err:
            print("Exception: {}".format(err))
            break;
