import pandas as pd

def read_data():
    data_set = pd.read_html('forex-currency-symbols.html', header=0)
    for frame in data_set:
        for row in frame.itertuples():
            pass

    data_set = pd.read_html('forex-currency-codes.html', header=0)
    for frame in data_set:
        for row in frame.itertuples():
            print(row.Subdivision)


read_data()