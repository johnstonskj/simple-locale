import pandas as pd

def read_data_from(created_date):
    tl_frame = pd.read_csv('iso-639-1-names.csv', sep=',', header=0)
    tl_names = {}
    for row in tl_frame.itertuples():
        tl_names[row.ID] = {
            'english': row.English_Name,
            'indigenous': row.Indigenous_Name
        }

    macros = pd.read_csv('iso-639-3-macrolanguages_%s.tab' % created_date, sep='\t', header=0)
    macro_languages = {}
    for row in macros.itertuples():
        if row.I_Status == 'A':
            parent = row.M_Id
            if parent not in macro_languages:
                macro_languages[parent] = []
            child = row.I_Id
            if isinstance(child, str):
                macro_languages[parent].append(child)

    main_map = {}
    lookup_map = {}
    main = pd.read_csv('iso-639-3_%s.tab' % created_date, sep='\t', header=0)
    for row in main.itertuples():
        children = []
        if row.Scope == 'M':
            children = macro_languages[row.Id]
        main_map[row.Id] = {
            'id': row.Id,
            'name': row.Ref_Name,
            'b_id': row.Part2B,
            't_id': row.Part2T,
            'sid': row.Part1,
            'scope': row.Scope,
            'l_type': row.Language_Type,
            'children': children,
        }
        if isinstance(row.Part1, str):
            lookup_map[row.Part1] = row.Id
    return (tl_names, main_map, lookup_map)

scope_values = {
    'I': 'Individual',
    'M': 'MacroLanguage',
    'S': 'Special'
}

type_values = {
    'A': 'Ancient',
    'C': 'Constructed',
    'E': 'Extinct',
    'H': 'Historical',
    'L': 'Living',
    'S': 'Special'
}

def write_data_out(tl_names, main_map, lookup_map):
    print('/* generated from ISO-639 data files */')
    print('')
    print('fn create_lookup_table() -> HashMap<InfoString, LanguageInfo> {')
    print('    let mut table = HashMap::new();')
    for (id, linfo) in main_map.items():
        print('    table.insert("%s", LanguageInfo {' % id)
        print('    table.insert("%s", LanguageInfo {' % id)
        print('        code: "%s",' % id)
        print('        reference_name: "%s",' % linfo['name'].replace("'", "\\'").replace('"', '\\"'))
        print('        indigenous_name: %s,' % indigenous_name(linfo['sid'], tl_names))
        print('        other_names: %s,' % other_names(linfo['sid'], tl_names))
        print('        bibliographic_code: %s,' % optional_string(linfo['b_id']))
        print('        terminology_code: %s,' % optional_string(linfo['t_id']))
        print('        short_code: %s,'  % optional_string(linfo['sid']))
        print('        scope: LanguageScope::%s,' % scope_values[linfo['scope']])
        print('        l_type: LanguageType::%s,' % type_values[linfo['l_type']])
        print('        family_members: %s,' % optional_vector(linfo['children']))
        print('    });')
    print('    table')
    print('}')

    print('')
    print('fn create_id_lookup_table() -> HashMap<InfoString, InfoString> {')
    print('    let mut table = HashMap::new();')
    for (sid, id) in lookup_map.items():
        print('    table.insert("%s", "%s");' % (sid, id))
    print('    table')
    print('}')

def other_names(key, map):
    if isinstance(key, str) and key in map:
        names =  map[key]['english'].split(';')
        if len(names) > 1:
            return optional_vector(names[1:])
    return 'None'

def indigenous_name(key, map):
    if isinstance(key, str) and key in map:
        return 'Some("%s")' % map[key]['indigenous'].replace("'", "\\'").replace('"', '\\"')
    else:
        return 'None'

def optional_string(s):
    return ('Some("%s")' % s) if isinstance(s, str) else 'None'

def optional_vector(v):
    if len(v) == 0:
        return 'None'
    else:
        return 'Some(vec![%s])' % ', '.join(list(map(lambda x: '"%s"' % x.strip().replace("'", "\\'").replace('"', '\\"'), v)))

write_data_out(*read_data_from('20190408'))