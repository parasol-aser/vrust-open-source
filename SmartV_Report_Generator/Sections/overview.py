import os
from datetime import date
from Sections.Findings import genStatistic

def genOverview(e):
    res = "\n\n# Overview  \n"
    res += "## Project Summary\n"
    res += '''| Project Name | $username$       | 
|--------------|----------| 
| Platform     | Ethereum | 
| Language     | Solana | 
| Crate     | $crate_name$ |
| GitHub Location      |    $git$    | 
| sha256       | $sha$       | \n
     '''
    res = res.replace("$username$", e["user"])
    res = res.replace("$crate_name$", e["crate"])
    res = res.replace("$git$", e["git-loc"])
    res = res.replace("$sha$", e["sha256"])

    res += "\n\n## Audit Summary\n\n"
    res += '''| Delivery Date | $date$       |
|--------------|----------|
| Audit Methodology     | Static Analysis      |
| Key Components        |      |\n
'''
    today = date.today()

    # dd/mm/YY
    d1 = today.strftime("%m/%d/%Y")

    res = res.replace("$date$", str(d1))

    res += "## Vulnerability Summary\n\n"

#     res += '''**Vulnerability Level**|**Total**|**Pending**|**Declined**|**Acknowledged**|**Partially Resolved**|**Resolved**
# :-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:
# Critical|0|0|0|0|0|0
# Major|0|0|0|0|0|0
# Medium|1|0|0|0|0|1
# Minor|1|0|0|1|0|0
# Informational|16|0|0|7|2|7
# Discussion|0|0|0|0|0|0 
# \n
# '''


    stat = genStatistic(e)
    res += "**Vulnerability Level**|**Total**"
    res += "\n:-----:|:-----:"
    # res += "\nCritical|" + str(stat[0])

    res += "\n<span style=\"color:#C0392B\"> Critical</span>|" + str(stat[0])
    res += "\n<span style=\"color:#E74C3C\">Major</span>|" + str(stat[1])
    res += "\n<span style=\"color:#D35400\">Medium</span>|" + str(stat[2])
    res += "\n<span style=\"color:#F5B041\">Minor</span>|" + str(stat[3])
    res += "\n<span style=\"color:#3498DB\">Informational</span>|" + str(stat[4])
    res += "\n<span style=\"color:#FFFF00\">Discussion</span>|" + str(stat[5])
    res += "\n\n"
    
   
   

    return res

