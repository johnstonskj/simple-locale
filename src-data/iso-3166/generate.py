import math
import pandas as pd

def read_data():
    data_frame = pd.read_csv('all.csv', header=0)

    regions = {}

    countries = {}

    for row in data_frame.itertuples():
        regions[int(row.country_code)] = row.name
        if not math.isnan(row.region_code):
            regions[int(row.region_code)] = row.region
        if not math.isnan(row.sub_region_code):
            regions[int(row.sub_region_code)] = row.sub_region
        if not math.isnan(row.intermediate_region_code):
            regions[int(row.intermediate_region_code)] = row.intermediate_region

        countries[row.alpha_3] = {
            'code': row.alpha_3,
            'short': row.alpha_2,
            'country': int(row.country_code),
            'region': None if math.isnan(row.region_code) else int(row.region_code),
            'sub_region': None if math.isnan(row.sub_region_code) else int(row.sub_region_code),
            'intermediate': None if math.isnan(row.intermediate_region_code) else int(row.intermediate_region_code)
        }
    return (regions, countries)

def write_data(regions, countries):
    print('/* generated from ISO-3166 data files */')
    print('')

    print('fn create_region_table() -> HashMap<u16, &\'static Region> {')
    print('    let mut table = HashMap::new();')
    for (code, name) in regions.items():
        print('    table.insert(%d, &Region {' % code)
        print('        code: %d,' % code)
        print('        name: "%s",' % name)
        print('    });')
    print('    table')
    print('}')

    lookup_map = {}
    LIMIT=25
    counter = 0
    for (code, cinfo) in countries.items():
        if counter % LIMIT == 0:
            if counter > 0:
                print('}')
            print('fn add_to_lookup_table_%d(table: &mut HashMap<InfoString, &\'static CountryInfo>) {' % int(counter / LIMIT))
        print('    table.insert("%s", &CountryInfo {' % cinfo['code'])
        print('        code: "%s",' % cinfo['code'])
        print('        short_code: "%s",' % cinfo['short'])
        print('        country_code: %d,' % cinfo['country'])
        print('        region_code: %s,' % ('None' if cinfo['region'] is None else 'Some(%s)' % cinfo['region']))
        print('        sub_region_code: %s,' % ('None' if cinfo['sub_region'] is None else 'Some(%s)' % cinfo['sub_region']))
        print('        intermediate_region_code: %s,' % ('None' if cinfo['intermediate'] is None else 'Some(%s)' % cinfo['intermediate']))
        print('    });')
        if isinstance(cinfo['short'], str):
            lookup_map[cinfo['short']] = cinfo['code']
        counter = counter + 1
    print('}')

    print('fn add_to_lookup_table_idx(table: &mut HashMap<InfoString, &\'static CountryInfo>) {')
    for (short, code) in lookup_map.items():
        print('    table.insert("%s", table.get("%s").unwrap());' % (short, code))
    print('}')

    print('fn create_country_table() -> HashMap<InfoString, &\'static CountryInfo> {')
    print('    let mut table = HashMap::new();')
    for fn in range(int(counter / LIMIT) + 1):
        print('    add_to_lookup_table_%d(&mut table);' % fn)
    print('    add_to_lookup_table_idx(&mut table);')
    print('    table')
    print('}')

write_data(*read_data())