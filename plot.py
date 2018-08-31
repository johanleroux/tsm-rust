import plotly.plotly as py
import plotly.graph_objs as go
import plotly.figure_factory as FF

import numpy as np
import pandas as pd

df = pd.read_csv('target/results.txt')

df_external_source = FF.create_table(df.head())

trace_fittest = go.Scatter(x = df['generation'], y = df['high'],
                  name='Fittest individual')

trace_average = go.Scatter(x = df['generation'], y = df['avg'],
                  name='Average individual')

trace_median = go.Scatter(x = df['generation'], y = df['median'],
                  name='Median individual')

layout = go.Layout(title='TSM - Tournament Selection',
                   plot_bgcolor='rgb(255, 255, 255)')

fig = go.Figure(data=[trace_fittest, trace_average, trace_median], layout=layout)

py.iplot(fig, filename='TSM - Tournament Selection')