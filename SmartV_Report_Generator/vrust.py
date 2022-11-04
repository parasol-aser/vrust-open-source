import argparse

parser = argparse.ArgumentParser()
parser.add_argument("dir", help="Input json_file.json")

parser.add_argument("--dest", help="/dir/to/report_destination")
parser.add_argument("--output", help="output_file.pdf")
parser.add_argument("--vrust", help="Location of vrust reporter")

args = parser.parse_args()

import pypandoc
import json
from datetime import date
import os
import subprocess
import sys
from datetime import datetime, timezone

from Sections.summary import genSummary
from Sections.overview import genOverview
from Sections.Findings import genFindings
from Sections.errorReport import genErrReport
from Sections.appendix import genAppendix
from Sections.disclaimer import genDisclaimer

newpage = '\n<div style="page-break-after: always;"></div>\n'
# SmartVLocation = "/home/siwei/SmartV_Report_Generator/" # does not support underscores in file path
SmartVLocation = "./"
DefaultLoc = '/home/ubuntu/VRust/SmartV_Report_Generator/'


def genTitlepage(user, reportdate):
    # today = date.today()

    # dd/mm/YY
    # d1 = today.strftime("%d/%m/%Y")
    d = datetime.fromtimestamp( int(reportdate), tz=timezone.utc  )
    d1 = d.strftime("%m/%d/%Y %H:%M:%S")
    res = "---\ntitle: \"Security Assessment\"\nauthor: ["+ user +"]\ndate: \"" + str(d1)+"\"\n"
    # res += "toc-own-page: true,\nheader-left: \"![SmartVLogo](./bg/SmartVbg.png)\",\n"
    # res += "header-right: \"Security Assessment\"\n"
    res += "page-background-opacity: 0.9\n"
    res += "page-background: \"./bg/VRust/VRustPageBG.png\"\n"
    # res += "keywords: [Markdown, Example]\n" # subtitle: "+ user 
    res += '''lang: "en"\ntitlepage: true\ntitlepage-text-color: "FFFFFF"\ntitlepage-rule-color: "360049"\ntable-use-row-colors: true\n
titlepage-rule-height: 0\ntitlepage-background: '''
    res += '"' +SmartVLocation + "bg/VRust/VRust.png\"\n...\n\n"
    return res

if __name__ == "__main__":
    print("VRust Report Generator")
    print("======================")
    print("Usage: python3 vrust.py /dir/to/json_file.json [/dir/to/report_destination] [-o output_file.pdf]")
    print("")

    # dir = "./report.json"
    dir = args.dir
    outputdir = "./"
    
    if args.dest:
        outputdir = args.dest
    
    if args.output:
        outputfile = args.output
    else:
        outputfile = "report.pdf"

    if args.vrust:
        DefaultLoc = args.vrust



    
    # if len(sys.argv)==2:
    #     dir = sys.argv[1]
    #     outputdir = dir.split("/")[:-1]
    #     outputdir = "/".join(outputdir)
    #     outputdir = outputdir + "/"
    # if len(sys.argv)==3:
    #     dir = sys.argv[1]
    #     outputdir = sys.argv[2]
    # outputfile = "report.pdf"
    # if len(sys.argv)==4:
    #     dir = sys.argv[1]
    #     if sys.argv[2] == "-o":
    #         outputdir = dir.split("/")[:-1]
    #         outputdir = "/".join(outputdir)
    #         outputdir = outputdir + "/"
    #         outputfile = sys.argv[3]
    #     else:
    #         print("Invalid argument")
    #         print("Usage: python3 vrust.py /dir/to/json_file.json [/dir/to/report_destination] [-o output_file.pdf]")
    #         print(sys.argv[2])
    #         exit()

    while not os.path.exists(dir):
        print("Please input json:")
        dir = input()

    while not os.path.exists(outputdir):
        print("Please provide output directory :")
        outputdir = input()

    with open(dir, "r") as f:
        jsondata = f.read()

    if outputdir.startswith("./"):
        outputdir = os.getcwd() + outputdir[1:]
    print("outputdir:", outputdir)
    # print(pypandoc.convert_text(json.dumps(jsondata), 'json', 'md'))
    # exit(0)

    try:
        report = json.loads(jsondata)
    except:
        print("Error: JSON file is not valid")
        exit(1)

    if "user" not in report:
        report["user"] = "O2Lab VRust Team"
    if "git-loc" not in report:
        report["git-loc"] = "Unknown"
    if "sha256" not in report:
        report["sha256"] = "Unknown"
    report["originLoc"] = outputdir # add search space under current folder
    
    # print(os.getcwd())
    os.chdir(DefaultLoc) # change to default location. This is to avoid the issue of relative path in markdown file.
    user = report["user"] 
    # print(report)
    markdown = genTitlepage(user, report["timestamp"]) 
    markdown += genSummary(user)  # working on now
    markdown += genOverview(report)
    markdown += genFindings(report)
    errs = report["errors"]
    for e in errs:
        markdown += genErrReport(e,report)
    # print(markdown)
    markdown += genAppendix()
    markdown += genDisclaimer()
    

    with open("./report.md", "w") as f:
        f.write(markdown)
    # output = pypandoc.convert_file('report.md', 'md', outputfile="report.pdf")
    # ret = os.system("pandoc " + "report.md -o "+ outputdir + "/report.pdf --from markdown --template " + SmartVLocation + "tex/eisvogel.latex --toc --listings --include-in-header "+SmartVLocation+"tex/chapter_break.tex ")
# --highlight-style tango  kate    
# listings

    # https://tex.stackexchange.com/questions/472127/pandoc-listings-breaks-styling-of-code-block
    # Highlighted in red, wrong line numbers
    # ret = os.system("pandoc " + " --filter ./vrust/pandoc-emphasize-code report.md -o "+ outputdir + "/" + outputfile + " --from markdown --template " + SmartVLocation + "tex/eisvogel.latex --toc --highlight-style tango --include-in-header "+SmartVLocation+"tex/chapter_break.tex --include-in-header " + SmartVLocation+"tex/emphasis.tex --listings" )
    # No line numbers, highlighter in yellow
    # ret = os.system("pandoc " + " --filter ./vrust/pandoc-emphasize-code report.md -o "+ outputdir + "/" + outputfile + " --from markdown --template " + SmartVLocation + "tex/eisvogel.latex --toc --highlight-style tango --include-in-header "+SmartVLocation+"tex/chapter_break.tex --include-in-header " + SmartVLocation+"tex/emphasis.tex" )
    # Rust code format
    ret = os.system("pandoc " + "  report.md -o "+ outputdir + "/" + outputfile + " --from markdown --template " + SmartVLocation + "tex/eisvogel.latex --toc --highlight-style tango --include-in-header "+SmartVLocation+"tex/chapter_break.tex --include-in-header " + SmartVLocation+"tex/voidemphasis.tex " )

# -V geometry:\"left=1cm, top=1cm, right=1cm, bottom=2cm\"

# --listings -H tex/listings-setup.tex 


#     pandoc --list-highlight-styles
# pygments
# tango
# espresso
# zenburn
# kate
# monochrome
# breezedark
# haddock
    if ret==0: 
        print(("Reported generated at: "+ outputdir + "/"+ outputfile).replace("//","/"))
    else:
        print("Error generating report")