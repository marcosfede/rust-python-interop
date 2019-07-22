import pandas as pd
from nltk.metrics import edit_distance
import rust_cpython_strd
import pyo3_strd


def string_similarity_table(values, metric=edit_distance):
    rows = []
    for i in range(len(values)):
        for j in range(i+1, len(values)):
            rows.append([values[i], values[j]])
    crossed = pd.DataFrame(rows, columns=['values_x', 'values_y'])
    crossed['distance'] = crossed.apply(
        lambda row: metric(row['values_x'], row['values_y']), axis=1)
    # remove same tag rows
    crossed = crossed[crossed.values_x != crossed.values_y]
    crossed.sort_values('distance', inplace=True)
    return crossed


df = pd.read_csv('./worldcities.csv')
data = df['city_ascii'].drop_duplicates().values
data.shape

limit = 1500
data_small = data[:limit]

%time crossed_exp = string_similarity_table(data_small)

%time string_similarity_table(data_small, metric=rust_cpython_strd.edit_distance)

%time string_similarity_table(data_small, metric=pyo3_strd.edit_distance)
