import math
import pandas as pd
import re
import sys

def read_data():
    frame = pd.read_excel('list_one.xls', skiprows=3, header=0)
    currencies = []
    for row in frame.itertuples():
        currencies.append({
            'alphabetic_code': row._3,
            'name': row.Currency,
            'standards_entity': row.ENTITY,
            'numeric_code': row._4,
            'symbol': None,
            'countries': [],
            'minor_units': row._5
        })

    sub_divisions = {}
    data_set = pd.read_html('forex-currency-codes.html', header=0)
    for frame in data_set:
        for row in frame.itertuples():
#            print('%s -> %s' % (row.Alphabetic_code, row.Subdivision))
            sub_divisions[row.Alphabetic_code] = row.Subdivision

    symbols = {}
    frame = pd.read_csv('currency-symbols-ex.csv', header=0)
    for row in frame.itertuples():
#        print('%s -> %s' % (row._3, row._6))
        symbols[row._3] = row._6

    return (currencies, sub_divisions, symbols)

def write_data_out(currencies, sub_divisions, symbols, out_path):
    rows = map(
        lambda cinfo:
        '"%s":{%s}' % (
            cinfo['alphabetic_code'],
            ','.join([
                '"alphabetic_code":"%s"' % cinfo['alphabetic_code'],
                '"name":"%s"' % cinfo['name'],
                '"standards_entity":"%s"' % clean(cinfo['standards_entity']),
                '"numeric_code":%s' % optional_number(cinfo['numeric_code']),
                '"symbol":%s' % optional_string(cinfo['symbol']),
                '"countries":[]',
                '"subdivisions":[%s]' % sub_division_list(sub_divisions.get(cinfo['alphabetic_code'], math.nan), cinfo['minor_units'])
            ])),
        currencies)
    for row in rows:
        print(row)
#     print('writing %s/currencies.json' % out_path)
#     with open('%s/currencies.json' % out_path, 'w') as text_file:
#         print('{%s}' % ','.join(rows), file=text_file)

def clean(s):
    return s.strip().replace('"', r'\"')

def optional_number(n):
    return 'null' if math.isnan(n) else int(n)

def optional_string(s):
    return ('"%s"' % s) if isinstance(s, str) else 'null'

def sub_division_list(subs, minor):
    sub_divs = []
    if not isinstance(subs, str) and isinstance(minor, str) and minor.isdigit():
        sub_divs.append('{"exponent":%s,"name":null}' % minor)
    elif isinstance(subs, str):
        if not subs  == '-none-' and not subs.endswith('used)'):
            parts = subs.split('(')
            parts_2 = parts[0].strip().split(' ')
            exponent = parts_2[0].count('0')
            if parts_2[1] == 'new':
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, 'new %s' % parts_2[2]))
            else:
                # rappen/centimes
                names = parts_2[1].split('/')
                for name in names:
                    sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, name))
            if len(parts_2) == 5 and parts_2[2] in ['=', 'or']:
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (parts_2[3].count('0'), parts_2[4]))
            another = '' if len(parts) == 1 else parts[1]
            if another.startswith('a.k.a'):
                another = another[1][7:-1]
            elif len(another) > 2:
                another = another[:-1]
            if another:
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, another))
    return ','.join(sub_divs)

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data_out(*read_data(), sys.argv[1])
