import os
import glob
import csv

extension = 'csv'
result = [i for i in glob.glob('target/test_rand_elt10_*.{}'.format(extension))]

gens = 0
best = 0
avg = 0

def row_count(filename):
    with open(filename) as in_file:
        return sum(1 for _ in in_file)

for i, file in enumerate(result):
    last_line_number = row_count(file)
    with open(file) as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            if last_line_number == reader.line_num:
                gens += int(row['gen'])
                best += float(row['best'])
                avg += float(row['avg'])

print(gens/10)
print(best/10)
print(avg/10)