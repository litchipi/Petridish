#!/usr/bin/env python3
#-*-encoding:utf-8*-

import warnings
warnings.filterwarnings("ignore", category=DeprecationWarning)

import sys, json, os, time
import genalgo as g

RESHEADER = ""
DBGHEADER = "D| "
ERRHEADER = "!!!> "
NOHEADER = "\n"

SCREEN_BUFFER = ""

def plot_scores(scores):
    import pylab
    import matplotlib.pyplot as plt
    fig = plt.figure()
    ax = fig.add_subplot(2, 1, 1)

    line, = ax.plot(scores, color='blue', lw=2)

    ax.set_yscale('log')

    pylab.show()

def res(msg):
    global SCREEN_BUFFER
    SCREEN_BUFFER += RESHEADER + msg + NOHEADER

def dbg(msg):
    global SCREEN_BUFFER
    SCREEN_BUFFER += DBGHEADER + msg + NOHEADER

def err(msg):
    raise Exception(msg)

def update_screen():
    os.system("clear")
    global SCREEN_BUFFER
    print(SCREEN_BUFFER)
    SCREEN_BUFFER = ""

if len(sys.argv) == 1:
    print("Arg: number of the test to launch")
    sys.exit(0)
else:
    TEST_NB = int(sys.argv[1])

def test(obj):
    dbg("Test nb " + str(TEST_NB))
    if TEST_NB == 0:
        plot_scores(test_simple_optimisation(obj))
    if TEST_NB == 1:
        res = list()
        for i in range(50):
            res.append(test_simple_optimisation(obj)[-1])
        print(sum(res)/len(res))

def test_simple_optimisation(obj, n=30000, nskip=2, nrefresh=1):
    data = json.dumps({"data":[]})
    obj.init_algo()
    scores = list()
    t = time.time()
    try:
        for i in range(n):
            dt = time.time() - t
            t2 = time.time()
            nbrecv = obj.receive_data(data)
            if obj.run_on_data():
                epoch = obj.finish_generation()
                dt2 = time.time()-t2
                t = time.time()
                if epoch != (i+1):
                    err("Epoch {} != {}, bizarre".format(i, epoch))
            else:
                err("Generation not finished, bizarre")
            if i < nskip:
                continue
            bestcell = json.loads(obj.save_bestcell_to_json())
            score = bestcell["score"]
            scores.append(score)
            if not (i%nrefresh == 0):
                continue
            res("{}> Best score got: {} (avg in {} secs)".format(obj.epoch, score, obj.get_avg_process_time()))
            #for g in bestcell["genome"]:
            #    res("{}: {}".format(str(g).ljust(30), abs(g-0.5)))
            res("Difference from optimum point: {}".format(sum([abs(g-0.5) for g in bestcell["genome"]])))
            res("Python code run in {} secs".format(dt))
            res("Rust code run in {} secs".format(dt2))
            update_screen()
    except KeyboardInterrupt:
        pass
    finally:
        with open("/tmp/opti_results.json", "w") as f:
            json.dump(scores, f)
        dbg("Finished")
        return scores


obj = g.create("benchmark")
cfg = json.loads(obj.save_json_config())
cfg["maximize_score"] = False
cfg["max_nb_cells"] = None
cfg["max_mem_used"] = 5*1024*1024
obj.load_json_config(json.dumps(cfg))
input("Number of cells: {}".format(obj.get_cells_number()))
test(obj)
