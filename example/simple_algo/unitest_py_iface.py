#!/usr/bin/env python3
#-*-encoding:utf-8*-

import sys
import json
import genalgo

def test0():
    lab_options = json.loads(genalgo.get_lab_default())
    lab_options["npop"] = 10000
    obj = genalgo.create_lab_test(json.dumps(lab_options))
    algo_a_ind = obj.register_algo_A()
    algo_config = genalgo.get_algo_default()
    obj.configure_algo(algo_a_ind, algo_config)
    obj.register_empty_dataset(3)
    for i in range(500):
        g, s = obj.start(1000)
        print(g, s)

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
