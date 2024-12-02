print(
    [
        f
        for f in zip(
            *[
                item.strip().split("   ")
                for item in open("./input/input_day01.txt").readlines()[:5]
            ]
        )
    ]
)

vals = open("./input/input_day01.txt").readlines()[:5]
for item in vals:
    print(int(item.split("   ")[0]))
