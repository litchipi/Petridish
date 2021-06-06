#!/usr/bin/env python3
#-*-encoding:utf-8*-

import sys
import json
import genalgo

NGENS=100
NITER=2
NPOP=5000

def create_lab_object():
    lab_options = json.loads(genalgo.get_lab_default())
    lab_options["npop"] = NPOP
    lab = genalgo.create_lab_test(json.dumps(lab_options))
    lab.register_empty_dataset(1)
    return lab

def run_lab(lab):
    for i in range(NITER):
        g, s = lab.start(NGENS)
        print(g, s)

def test_map_assistant(n=5):
    obj = genalgo.create_labmap_assistant("WheelFormat")
    for i in range(n):
        obj.add_opti_part("ISO" + chr(ord("A") + i),
                [i], 0.6 + (i/10), "Darwin",
                '{"DarwinConfig":{"gene_reroll_proba":0.5,"exploration_scope_epoch_max":3}}')
    res = json.loads(obj.generate_map("Darwin"))
    print("{} algos".format(len(res)))
    for nb, r in enumerate(res):
        print("Algo n°{}:\tid {}, give: {}, impr_genes: {}, population: {}".format(
            nb, r["id"], r["give"], r["impr_genes"], r["population"]
            ))

def test_apply_map(ngens=4):
    lab = create_lab_object()
    labast = genalgo.create_labmap_assistant("WheelFormat")
    for i in range(0, ngens, 2):
        labast.add_opti_part("ISO" + chr(ord("A") + i),
                [i, i+1], 0.6 + (i/10), "Darwin",
                '{"DarwinConfig":{"gene_reroll_proba":0.5,"exploration_scope_epoch_max":3}}')
    labmap = json.loads(labast.generate_map("Darwin"))
    for i in range(len(labmap)):
        if "Mix" not in labmap[i]["id"]:
            algo_a_ind = lab.register_algo_A()
        else:
            algo_b_ind = lab.register_algo_B()
        if labmap[i]["id"] == "Final":
            lab.set_output_algorithm(i)

    lab.apply_map(json.dumps(labmap))
    run_lab(lab)

def test_create_from_map(ngens=4):
    lab = create_lab_object()
    labast = genalgo.create_labmap_assistant("WheelFormat")
    for i in range(0, ngens, 2):
        labast.add_opti_part("ISO" + chr(ord("A") + i),
                [i, i+1], 0.6 + (i/10), "Darwin",
                '{"DarwinConfig":{"gene_reroll_proba":0.5,"exploration_scope_epoch_max":3}}')
    labmap = json.loads(labast.generate_map("Darwin"))
    for i in range(len(labmap)):
        if labmap[i]["id"] == "Final":
            lab.set_output_algorithm(i)
            break

    lab.apply_map_with_algo_A(json.dumps(labmap))
    run_lab(lab)


def test_empty_opti():
    obj = create_lab_object()
    algo_a_ind = obj.register_algo_A()
    algo_config = genalgo.get_algo_default()
    obj.configure_algo(algo_a_ind, algo_config)
    run_lab(obj)

ALL_TESTS = [
        test_empty_opti,

        # LAB MAPS
        test_map_assistant,
        test_apply_map,
        test_create_from_map,
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
