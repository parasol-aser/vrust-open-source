import os

def genSummary(user):
    res = "# Summary  \n\n"
    res += "This report has been prepared for "+ user + " to discover issues and vulnerabilities in the source code of the "
    res += user +" project as well as any contract dependencies that were not part of an officially recognized library. A "
    res += "comprehensive examination has been performed, utilizing Static Analysis and Manual Review techniques.\n"

    res += "The auditing process pays special attention to the following considerations:"
    res += '''\n\n\n\n* Testing the smart contracts against both common and uncommon attack vectors. 
                \n\n* Assessing the codebase to ensure compliance with current best practices and industry standards.
                \n\n* Ensuring contract logic meets the specifications and intentions of the client.
                \n\n* Cross referencing contract structure and implementation against similar smart contracts produced
                by industry leaders.
                \n\n* Thorough line-by-line manual review of the entire codebase by industry experts. \n\n '''

    res += '''The security assessment resulted in findings that ranged from critical to informational. We recommend
addressing these findings to ensure a high level of security standards and industry practices.
We suggest
recommendations that could better serve the project from the security perspective:
'''

    res += '''\n\n*  Enhance general coding practices for better structures of source codes;
\n\n* Add enough unit tests to cover the possible use cases;
\n\n* Provide more comments per each function for readability, especially contracts that are verified in
public;
\n\n* Provide more transparency on privileged activities once the protocol is live.
    
    '''
    return res

if __name__ == "__main__":
    res = genSummary("user")
    print(res)
