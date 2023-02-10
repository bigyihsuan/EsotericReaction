import csv
import dataclasses


@dataclasses.dataclass
class Element:
    n: int
    name: str
    symbol: str
    bonds: int


data: list[Element] = []
header: list[str] = []

with open("elements.csv", "r") as file:
    reader = csv.DictReader(file, delimiter=',', fieldnames=["n", "symbol", "name", "bonds"])
    h = next(reader, {})
    header = [k for k in h.keys()]
    for d in reader:
        data.append(Element(n=int(d['n']), symbol=d['symbol'], name=d['name'], bonds=0))

for i, e in enumerate(data):
    if 1 <= e.n <= 3:
        e.bonds = 2
    elif 21 <= e.n <= 30 or \
            39 <= e.n <= 48 or \
            72 <= e.n <= 80 or \
            104 <= e.n <= 112 or \
            57 <= e.n <= 71 or \
            89 <= e.n <= 103:
        e.bonds = 18
    else:
        e.bonds = 8
    data[i] = e

with open("elements-bonds.csv", "w") as file:
    writer = csv.writer(file)
    writer.writerow(header)
    for d in data:
        writer.writerow([d.n, d.symbol, d.name, d.bonds])
