

import os



margin = 5

print("[!] Code Margin set to ", margin)

def code_margin(code, var, start_line_no):

    res = ""

    # Handle multi-line code
    if "\n" in var:
        var = var.split("\n")[0]

    ## Handling function header



    if "{" in code and code.strip().startswith("fn"):
        func_header = code.split("{")[0]
        res += "-- Function Definition: \n\n" + "```{.rust .numberLines startFrom=\""
        res += str(start_line_no) + "\"}\n" + func_header + "\n"
        res += "\n\n```\n\n"
        

    start_line_no = int(start_line_no)

    lines = code.split("\n")

    line_with_var = []
    for i in range(len(lines)):
        if var in lines[i]:
            line_with_var.append(i)
    
    new_start_line_no = max(0, line_with_var[0] - margin) # exclude offset
    new_end_line_no = min(len(lines), line_with_var[0]+margin)  # exclude offset

    res += "\n\nVulnerability at Line: " + str(start_line_no+line_with_var[0]) + "\n\n"
    res += "```{.rust .numberLines startFrom=\""+ str(start_line_no + new_start_line_no) +"\"}\n"
    for i in range(new_start_line_no, new_end_line_no):
        res += lines[i] + "\n"
        # res += str(i) + ": " + lines[i-1]
    res += "\n\n```\n\n"

    # Add data flow
    tres = ""
    tres += "\n\nOther Use Case for Variable: " + var + "\n\n"
    oth_cases = False
    for i in range(1,len(line_with_var)):

        if lines[line_with_var[i]].strip().startswith("//"):
            continue
        oth_cases = True
        tres += "\n\n"
        tres += "```{.rust .numberLines startFrom=\""+ str(line_with_var[i]+start_line_no) +"\"}\n"
        tres += lines[line_with_var[i]]
            # res += str(i) + ": " + lines[i-1]
        tres += "\n\n```\n\n"
        tres += "\n\n"
    if oth_cases:
        res += tres
    

    if len(res) == 0:
        print("[!] Error: Code Margin not found. Include the whole function")
        res += "```{.rust .numberLines startFrom=\""+ str(start_line_no) +"\"}\n"
        res += code + "\n\n```\n\n"
    else:
        print("[!] Code Margin found")
        # print(res)

    return res



def genCode(e,report):
    res = ""
    if "variable" in e and len(e['variable'])>0:
        res += "* Variable\n\n"
        res += "`" + e['variable'] + "`\n\n"
        res += "\n\n"

    res += "* Location\n\n"
    v = e['location']
    
    if len(e["code"]) > 1:
        data = e["code"]
        if "\n\t" in data:
            path = data.split("\n\t")[0]
            code = "\n\t".join(data.split("\n\t")[1:])
            res += path + "\n\n"
            start_line_no = path.split(":")[1]
            if "variable" in e and len(e["variable"])>0 and e["variable"] in code:
                res += code_margin(code, e['variable'], start_line_no)
            else:
                print("[!]  Warning: variable not found. Including the whole function in the report")
                res += "```{.rust .numberLines startFrom=\""+ start_line_no +"\"}\n"
                res += code + "\n"
                res += "\n\n```\n\n"
        else:
            res += "```{.rust }\n"
            res += data + "\n"
            res += "\n\n```\n\n"

        # Handle context
        if "context" in e and len(e["context"])>0:

            res += "\n\n* Code Context\n\n"
            data = e["context"]
            # path_cont, code = data.split("\n\t")
            path_cont = data.split("\n\t")[0]
            code = "".join(data.split("\n\t")[1:])
            start_line_no_context = path_cont.split(":")[1]

            # Including only margin
            # If the context is involved, the code must be the statement

            if "\n\t" in e["code"]:
                tcode = e["code"].split("\n\t")[1]
                if tcode in code and len(tcode) > 0:
                    # print("[!] DEBUG: ", tcode, " in ", code)
                    res += code_margin(code, tcode, start_line_no_context)
            else:
                print("[!] Warning: Context not found. Include the whole function")
                res += "```{.rust .numberLines startFrom=\""+ start_line_no_context +"\"}\n"
                res += code + "\n"
                res += "\n\n```\n\n"


            # Including the whole function in the report
            # res += path_cont + "\n\n"
            # 
            # emphasize = ""
            # if len(path.split(":")) == 5:
            #     _, st, stc, ed, edc = path.split(":")
            #     if int(st) >= int(start_line_no_context):
            #         nst = int(st) - int(start_line_no_context) + 1
            #         ned = int(ed) - int(start_line_no_context) + 1
            #         emphasize = str(nst) + ":" + stc.strip() + "-" + str(ned) + ":" + edc.strip()
            # else:
            #     print(len(path.split(":")))
            #     print(path.split(":"))
            # # print("emphasize = " + emphasize)
            # res += "```{.rust .numberLines startFrom=\""+ start_line_no_context +"\" emphasize=" + emphasize + "}\n"
                
            # res += code + "\n"
            # res += "\n\n```\n\n"


        return res

    if "rs" in v:
        data = v.split("\n")[0]
        res += data + "\n\n"
        if len(data.split(":")) == 2:
            loc, line = data.split(":")
            line = int(line)
            if not os.path.isfile(loc):
                newloc = os.path.join(report["originLoc"], loc)
                print("newloc:", newloc)
                if os.path.isfile(newloc):
                    loc = newloc
                else:
                    if "local-loc" in report:
                        if os.path.isfile(report["local-loc"]):
                            loc = report["local-loc"]
                        else:
                            pass
                    # print("[!] Found file: " + loc)
                    else:
                        print("[!] local-loc not found. ")
                        print(e)
            if os.path.isfile(loc):
                with open(loc, "r") as f:
                    lines = f.readlines()
                    if len(lines) > int(line):
                        diff = min(margin,line)
                        res += "```{.rust .numberLines startFrom=\""+ str(line-diff) +"\"}\n"
                        for i in range(line-diff, min(len(lines),line+margin)):
                            res += lines[i-1]
                            # res += str(i) + ": " + lines[i-1]
                        res += "\n\n```\n\n"
                return res
            else:
                print("File not found: " + loc)
                return res + "\n\n" + e['location'] + "\n\n"
        else:
            print("Error: Invalid location format")
            return res + "\n\n" + e['location'] + "\n\n"

    else:
        print("No rust source code found.")
        return res
    
def genStack(stack):
    res = "* Call Stack\n\n"
    inc = 0
    res += "```{.rust .numberLines}\n"
    for s in stack.split("\n"):
        res += "".join([" " for i in range(inc)]) + s + "\n"
        inc += 1
    res += "```\n\n"

    return res

def genErrReport(e,report):
    # print(e)
    res = "\n# Issue: " + e['id'] + ": " + e['category']  + "\n"

    res += "| Category | Severity | Status |\n"
    res += "| :------- | :------- | :------ |\n"
    res += "| " + e['category'] + " | " + e['severity'] + " | " + e['status'] + " |\n\n"
    res += "\n"

    # lst = ["Title", "Category", "Severity", "Location", "Stack", "Description", "Status", "Recommendation", "Alleviation"]
    lst = ["location", "stack", "description", "link" , "alleviation"]
    for t in lst:
        if t == "location":
            res += genCode(e,report)
        elif t== "stack":
            res += genStack(e["callstack"])
        else:
            res += "* " + t + ": \n\n" + e[t] + "\n\n"
    # res += "* " + e["title"] + "\n\n" + v + "\n\n"
    # for k,v in e.items():
    #     if k in ['id','title']:
    #         continue
    #     if k == 'location':
    #         res += genCode(v,report)
    #     else:
    #         res += "* " + k + "\n\n" + v + "\n\n"
    return  res + "\n"
