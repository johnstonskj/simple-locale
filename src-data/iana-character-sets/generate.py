import math
import pandas as pd
import re
import sys

def read_data():
    data_frame = pd.read_csv('people.tsv', sep='\t', header=0)
    people = []
    for row in data_frame.itertuples():
        people.append(row._1)

    data_frame = pd.read_csv('character-sets-1.csv', header=0)

    character_sets = []

    for row in data_frame.itertuples():
        character_sets.append({
            'name': row.Name,
            'mid_code': row.MIBenum,
            'source': un_newline(row.Source),
            'reference': un_newline(row.Reference),
            'aliases': re.split("\n+", str(row.Aliases))
        })

    return (character_sets, people)

def write_data(character_sets, people, out_path):
    for row in character_sets:
        print(row['reference'])

def old_write():
    r_rows = map(
        lambda rinfo: '"%s":"%s"' % (rinfo[0], rinfo[1]),
        regions.items())
    print('writing %s/regions.json' % out_path)
    with open('%s/regions.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(r_rows), file=text_file)

    c_rows = map(
        lambda cinfo:
           '"%s":{%s}' % (
                cinfo['code'],
                ','.join([
                    '"code":"%s"' % cinfo['code'],
                    '"short_code":"%s"' % cinfo['short'],
                    '"country_code":%s' % cinfo['country'],
                    '"region_code":%s' % ('null' if cinfo['region'] is None else '%s' % cinfo['region']),
                    '"sub_region_code":%s' % ('null' if cinfo['sub_region'] is None else '%s' % cinfo['sub_region']),
                    '"intermediate_region_code":%s' % ('null' if cinfo['intermediate'] is None else '%s' % cinfo['intermediate'])
                ])),
        countries)
    print('writing %s/countries.json' % out_path)
    with open('%s/countries.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(c_rows), file=text_file)

def un_newline(s):
    return s.replace('\n', ' ') if isinstance(s, str) else s

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data(*read_data(), sys.argv[1])