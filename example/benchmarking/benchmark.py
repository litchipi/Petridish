#!/usr/bin/env python3
#-*-encoding:utf-8*-

import genalgo
import json

from threading import Event, Thread

NPOP = 1000000
UPDATE_GEN_NB = 100
NDATA = 1
NDIM = 5

def create_lab_dim(dim, lab_options):
    print(lab_options)
    if dim == 1:
        return genalgo.create_lab_dim1(json.dumps(lab_options))
    if dim == 5:
        return genalgo.create_lab_dim5(json.dumps(lab_options))

class Plot(Thread):
    def __init__(self):
        Thread.__init__(self)
        self.event = Event()
        self.stop = False
        self.scores = list()

    def update(self):
        if len(self.scores) > 0:
            print("Last score: {}".format(self.scores[-1]))
            #TODO   Live matplotlib-style plot to show score

    def run(self):
        while not self.stop:
            self.event.clear()
            self.event.wait()
            self.update()

#TODO   Clean this code, put as separate functions
#       Make a python executable with cli args to try things
#       Bench to test maps
#           -> Interactive map creator
#           -> Test how much iterations before getting a certain score
#               OR
#           -> Test score gotten after a certain number of iterations

MATH_FCT = [
        "spherical",        # Optimisation OK
        "xinsheyang1",      # Optimisation OK
        "xinsheyang2",      # Find only local minimums      #TODO   Find a fix
        "schwefel220",
        "styblinski_tank",
        "quartic"
        ]

MINIMUM = [0.5, 0.5, 0.5]

choose_msg = "Choose between math functions:\n"
for n, fct in enumerate(MATH_FCT):
    choose_msg += "\t{}> {}\n".format(n, fct)
choose_msg += "Choice: "

math_fct_nb = int(input(choose_msg))
lab_options = json.loads(genalgo.get_lab_default())
lab_options["npop"] = NPOP

obj = create_lab_dim(NDIM, lab_options)
algo_ind = obj.register_algo_benchmark()

algo_config = json.loads(genalgo.get_algo_default())
algo_config["method_options"]["DarwinConfig"]["optimization_ratio_epoch_shift"] = 1000
print(algo_config)
algo_config["method_options"]["DarwinConfig"]["gene_reroll_proba"] = 0.3
print(algo_config)
obj.configure_algo(algo_ind, json.dumps(algo_config))

obj.register_empty_dataset(NDATA)
obj.push_special_data(algo_ind, json.dumps({"mathfct":MATH_FCT[math_fct_nb], "scope":[-5, 5]}))
d = obj.get_special_data(algo_ind, json.dumps({"method":"expected_optimum", "scope":[-5, 5]}))

print(d)
MIN_COORD = json.loads(d)["result"]
print(MIN_COORD)
input("Enter to start")

plot = Plot()
try:
    plot.start()
    i = 0
    while True:
        i += UPDATE_GEN_NB
        g, s = obj.start(UPDATE_GEN_NB)
        print("\nGeneration {}".format(i))
        print("Best genome:      \t", g)
        print("Best score:       \t", s)
        print("Dist from minimum:\t", sum([abs(x-MIN_COORD[n]) for n, x in enumerate(g)]))
        plot.scores.append(s)
        plot.event.set()
finally:
    plot.stop = True
    plot.event.set()
