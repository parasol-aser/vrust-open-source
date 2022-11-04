from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

import argparse, difflib, PyPDF2, os, smtplib, ssl

#####      CMD ARGS      #####

parser = argparse.ArgumentParser()

parser.add_argument('-email', dest='sender_email', help='Email of the sender', required=True)
parser.add_argument('-recipients', dest='recipients', help='Emails of the receiver', nargs='*', required=True)
parser.add_argument('-pass', dest='sender_pass', help='Password to sender email', required=True)

parser.add_argument('-prev', dest='prev_path', help='Directory path of previous .pdf reports', required=True)
parser.add_argument('-curr', dest='curr_path', help='Directory path of current .pdf reports', required=True)

parser.add_argument('-s', dest='server', help='Email server for SMTP', default='smtp.gmail.com')
parser.add_argument('-p', dest='port', help='Email server port for SMTP', default=587)

args = parser.parse_args()

#####     CONSTANTS     #####

PREV_FOLDER = os.path.normpath(args.prev_path)
CURR_FOLDER = os.path.normpath(args.curr_path)

d = difflib.HtmlDiff()

EMAIL = args.sender_email
PASSWORD = args.sender_pass
RECIPIENTS = args.recipients

SERVER = args.server
PORT = args.port

HTML = '''\
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
          "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html>
<head>
    <meta http-equiv="Content-Type"
          content="text/html; charset=utf-8" />
    <title></title>
    <style type="text/css">
        table.diff {font-family:Courier; border:medium;}
        .diff_header {background-color:#e0e0e0}
        td.diff_header {text-align:right}
        .diff_next {background-color:#c0c0c0}
        .diff_add {background-color:#aaffaa}
        .diff_chg {background-color:#ffff77}
        .diff_sub {background-color:#ffaaaa}
    </style>
</head>
<body>
    <p>This is a report in finding the differences of generated .pdf reports from {prev_folder} and in {curr_folder}.</p>
{body}
    <table class="diff" summary="Legends">
        <tr> <th colspan="2"> Legends </th> </tr>
        <tr> <td> <table border="" summary="Colors">
                      <tr><th> Colors </th> </tr>
                      <tr><td class="diff_add">&nbsp;Added&nbsp;</td></tr>
                      <tr><td class="diff_chg">Changed</td> </tr>
                      <tr><td class="diff_sub">Deleted</td> </tr>
                  </table></td>
             <td> <table border="" summary="Links">
                      <tr><th colspan="2"> Links </th> </tr>
                      <tr><td>(f)irst change</td> </tr>
                      <tr><td>(n)ext change</td> </tr>
                      <tr><td>(t)op</td> </tr>
                  </table></td> </tr>
    </table>
</body>
</html>
'''

message = MIMEMultipart("alternative")
message["Subject"] = "[vrust-report] diff report"
message["From"] = EMAIL

##### UTILITY FUNCTIONS #####

def extract_report_issues(file_path):
    '''
    A function that returns a list of issues reported by mirav, given the file
    path in string format for the location of a generated .pdf report.
    '''
    fd = open(file_path, 'rb')
    file_reader = PyPDF2.PdfFileReader(fd)
    
    issues = []

    for page in range(1, file_reader.numPages + 1):
        page_obj = file_reader.getPage(page)
        page_text = page_obj.extractText()

        if 'Issue' not in page_text:
            break
        
        for line in page_text.split('\n'):
            if line.startswith('Issue:'):
                issues.append(line.strip())

    fd.close()
    return issues

def generate_report_body(file_name, prev_issues, curr_issues,):
    '''
    A function that returns an HTML string regarding the difference of two reports for
    a consistent e-mail format, given the program report's file name, and a list of 
    previous and current report issues.
    '''
    html_rtn = ''
    html_table = d.make_table(prev_issues, curr_issues)
    
    html_rtn += '\n<b>' + file_name + '</b>\n'
    html_rtn += html_table
    
    return html_rtn

#####    MAIN SCRIPT    #####

if not os.path.isdir(PREV_FOLDER) or not os.path.isdir(CURR_FOLDER):
    exit()

prev_pdfs = set(_ for _ in os.listdir(PREV_FOLDER) if _.endswith('.pdf'))
curr_pdfs = set(_ for _ in os.listdir(CURR_FOLDER) if _.endswith('.pdf'))
pdfs_to_check = prev_pdfs & curr_pdfs

body_html = ''

for file_name in pdfs_to_check:
    prev_file = os.path.normpath(PREV_FOLDER + '/' + file_name)
    curr_file = os.path.normpath(CURR_FOLDER + '/' + file_name)

    prev_issues = extract_report_issues(prev_file)
    curr_issues = extract_report_issues(curr_file)
    
    if prev_issues == curr_issues:
        continue

    body_html += generate_report_body(file_name, prev_issues, curr_issues)

if body_html == '':
    body_html = '<p><b>No differences found.</b></p>'

HTML = HTML.replace('{body}', body_html)
HTML = HTML.replace('{prev_folder}', PREV_FOLDER)
HTML = HTML.replace('{curr_folder}', CURR_FOLDER)
message.attach(MIMEText(HTML, 'html'))

context = ssl.create_default_context()
with smtplib.SMTP(SERVER, PORT) as server:
    server.starttls(context=context)
    server.login(EMAIL, PASSWORD)
    server.sendmail(EMAIL, RECIPIENTS, message.as_string())
