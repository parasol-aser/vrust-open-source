# SmartV_Report_Generator

Usage: python3 ./smartv.py name_of_the_json.json [--vrust  LOCATION_OF_THIS_REPO]





Input: report.json
Output: report.md, paper.pdf

Environment: Please look into env.txt.

## Margin

Setting margin:
https://github.com/jncsw/SmartV_Report_Generator/blob/a4df9971abd006f808e31bb9ddca72fa85628d15/Sections/errorReport.py#L7


To utilize this feature, you need to:

(1) (Integer CVE) Use `code` to specify one statement and put the whole function body to `context`  
(2) (Others) use `variable` for interested variable and `code` for the whole function body.


Demo:
Run `python3 ../vrust.py ./vrust_new.json`, and you will get:
https://github.com/jncsw/SmartV_Report_Generator/blob/VRust/vrust/margin_demo.pdf

## test case

```
siwei@ip-172-31-30-190:~/SmartV_Report_Generator$ cd test/
siwei@ip-172-31-30-190:~/SmartV_Report_Generator/test$ ls
addlocation.json  report.pdf  test1
siwei@ip-172-31-30-190:~/SmartV_Report_Generator/test$ code addlocation.json 
siwei@ip-172-31-30-190:~/SmartV_Report_Generator/test$ python3 ../smartv.py ./addlocation.json 
outputdir: /home/siwei/SmartV_Report_Generator/test/
newloc: /home/siwei/SmartV_Report_Generator/test/victim.sol
Reported generated at: /home/siwei/SmartV_Report_Generator/test/report.pdf
siwei@ip-172-31-30-190:~/SmartV_Report_Generator/test$ 

```

The sample report is produced in `test/report.pdf`.

