import os
from datetime import date
import matplotlib.pyplot as plt
import numpy as np
labels = ['Critical', 'Major', 'Medium', 'Minor', 'Informational', 'Discussion']
sizes = [0, 0, 1, 1, 16, 0]

def genFig1(e):
    
    
    class MyClass:
        i = -1

    def func(pct, labels, vals):
        MyClass.i +=1
        # Returns absolute value against the default percentage
        # absolute = int(pct/100.*np.sum(vals))
        # Combine labels and values
        return "{:s}\n{:.0f} %".format(labels[MyClass.i], pct)


    fig1, ax1 = plt.subplots()
    # Pie wedgeprops with width being the donut thickness
    ax1.pie(sizes, wedgeprops=dict(width=0.7), autopct=lambda pct: func(pct, labels, sizes),
            shadow=True, startangle=90)
    sumstr = 'Total Issues: '+str(np.sum(sizes))
    # String on the donut center
    ax1.text(0., 0., sumstr, horizontalalignment='center', verticalalignment='center')
    ax1.axis('equal')  # Equal aspect ratio ensures that pie is drawn as a circle.

    # plt.show()
    plt.savefig('./img/fig.png',scale =2.0)
def genFig2(e):

    import plotly.express as px
    import plotly.graph_objects as go
    colors = ['#C0392B', '#E74C3C', '#D35400', '#F5B041', '#3498DB', '#FFFF00']
    fig = go.Figure(data=[go.Pie(labels=labels, values=sizes)])
    # fig.show()
    # fig.update_traces(hole=.5, hoverinfo="label+percent+name")
    fig.update_traces(hole=.5, hoverinfo="label+percent+name", textinfo='value', textfont_size=20,
                  marker=dict(colors=colors, line=dict(color='#000000', width=2)))

    fig.update_layout(
    title_text="Bug Findings",
    # Add annotations in the center of the donut pies.
    annotations=[dict(text='Total Issues: '+str(np.sum(sizes)), x=0.5, y=0.5, font_size=20, showarrow=False)])
    fig.write_image("./img/fig.png",scale =4.0)
def genStatistic(e):
    # print(err['errors'])
    for i in range(len(sizes)):
        sizes[i] = 0
    for err in e['errors']:
        

        if err["severity"] == 'Critical':
            sizes[0] += 1
        elif err["severity"] == 'Major':
            sizes[1] += 1
        elif err["severity"] == 'Medium':
            sizes[2] += 1
        elif err["severity"] == 'Minor':
            sizes[3] += 1
        elif err["severity"] == 'Informational':
            sizes[4] += 1
        elif err["severity"] == 'Discussion':
            sizes[5] += 1
        
    return sizes

def genFindingTable(e):
    
    res = "\n\n| ID | Category | Severity | Status |\n"
    res += "| :--- | :--- | :--- | :---: |\n"
    for err in e['errors']:
        res += "| " + str(err["id"]) + " | " + err["category"] + " | " + err["severity"] + " | " + err["status"] + " |\n"
    res += "\n\n"

    return res

    
def genFindingStatistic(e):
    from collections import defaultdict
    res = "\n\n# Finding Statistic  \n\n"
    AllBugTypes = defaultdict(int)
    for err in e['errors']:
        AllBugTypes[err["category"]] += 1
    res += "| Category | Count |\n"
    res += "| :--- | :--- |\n"
    for key, value in AllBugTypes.items():
        res += "| " + key + " | " + str(value) + " |\n"

    

    return res + "\n\n"

def genFindings(e):

    
    genStatistic(e)

    genFig2(e)
    res = "\n\n# Findings  \n\n"
    res += "![Findings](./img/fig.png)"

    res += genFindingStatistic(e)
    res += genFindingTable(e)
   
    return res + "\n\n"

# if __name__ == "__main__":
#     res = genSummary("user")
#     print(res)
