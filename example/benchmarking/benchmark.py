#!/usr/bin/env python3
#-*-encoding:utf-8*-

import genalgo
import json

lab_options = json.loads(genalgo.get_lab_default())
lab_options["npop"] = 1000

obj = genalgo.create_lab_dim10(json.dumps(lab_options))
algo_a_ind = obj.register_algo_benchmark()
algo_config = json.loads(genalgo.get_algo_default())
obj.configure_algo(algo_a_ind, json.dumps(algo_config))
obj.register_empty_dataset(3)
#TODO   Use send_special_data / recv_special_data to setup algorithms
#TODO   Use matplotlib to display results       (possibility to it in live ?)
for i in range(500):
    g, s = obj.start(1000)
    print(g, s)
