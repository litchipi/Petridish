#!/usr/bin/env python3
#-*-encoding:utf-8*-

import genalgo

def test0():
    obj = genalgo.create_lab_test(genalgo.get_lab_default())
    obj.register_algo_A()
    obj.start(100)

def test1():
    obj = genalgo.create_lab_test(genalgo.get_lab_default())
    obj.register_algo_A()
    obj.register_algo_B()
    #TODO   Set output algorithm
    obj.start(100)

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
