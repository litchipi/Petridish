scores = [
        1428,
        102343,
        102432,
        1203,
        8950,
        395,
        755,
        120,
        121,
        65,
        46,
        23,
        12,
        14,
        8.25,
        5.24,
        4.9,
        4.88,
        2.2,
        2.05,
        1.2,
        1.0,
        0.5,
        0.07,
        0.03,
        0.0302,
        0.03025,
        0.03011,
        0.03010,
        0.00035,
        0.00034
        ]

scores = sorted(scores)

import math
TRANSFORM = {
        "ident": lambda x: x,
        "ln"   : lambda x: math.log(x),
        "log10": lambda x: math.log10(x),
        "log2" : lambda x: math.log2(x),        # Solution retained for now
        "log worst":lambda x: math.log(x, scores[-1]),
        "log avg best / worst":lambda x: math.log(x, (scores[0] + scores[-1])/2),
        }

print("".join([k.ljust(25) for k in TRANSFORM.keys()]))
for i in range(len(scores)):
    s = ""
    for f in TRANSFORM.values():
        sf = "{}".format(round(f(scores[i]), 5))
        if i == 0:
            diff = 0
        else:
            diff = f(scores[i]) - f(scores[i-1])
        sf += " ({})".format(round(diff, 5))
        s += sf.ljust(25)
    print(s + "|")
