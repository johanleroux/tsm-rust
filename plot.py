import plotly.plotly as py
import plotly.graph_objs as go
import plotly.figure_factory as FF
import plotly.io as pio

import numpy as np
import pandas as pd

import os
import glob

extension = 'csv'
result = [i for i in glob.glob('target/*.{}'.format(extension))]

for i, file in enumerate(result):
    df = pd.read_csv(file)

    df_external_source = FF.create_table(df.head())

    trace_fittest = go.Scatter(x = df['gen'], y = df['best'],
                    name='Fittest individual')

    trace_average = go.Scatter(x = df['gen'], y = df['avg'],
                    name='Average individual')

    layout = go.Layout(
        title='Tournament Selection (Elitism 1%)',
        plot_bgcolor='rgb(255, 255, 255)',
        autosize=False,
        yaxis={
        'type':'linear',
        'autorange':True,
        'title':'Distance'
        },
        showlegend=True,
        height=400,
        width=600,
        titlefont={
        'size':18
        },
        xaxis={
        'type':'linear',
        'autorange':True,
        'title':'Generation'
        },
        hovermode='closest',
        font={
        'size':12
        },
        margin={
        "pad": 0,
        "r": 0,
        "b": 40,
        "t": 30,
        "l": 50,
        }
    )

    fig = go.Figure(data=[trace_fittest, trace_average], layout=layout)
    pio.write_image(fig, file + '.png')

    #py.iplot(fig, filename='Tournament Selection (Elitism 1%) - ' + str(i))
