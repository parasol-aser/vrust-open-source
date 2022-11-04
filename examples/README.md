# vrust: Real World Projects
This directory currently contains builds of real world projects published on the Solana blockchain, obtained from the [anchor](https://anchor.projectserum.com) webpage, where all projects are demonstrated to be using the Anchor framework -- a Solana sealevel framework. All of the projects were found and tested successfully using vrust.

## run.py automation script
In addition to a stand-alone vrust tool and a set of real world projects is an automated python3 tool (run.py) to run vrust on all of the projects listed in this directory and exported to a reports directory. It requires >=Python3.6 and a toml package installed from pip.

To run the script, you must provide the location of the vrust build and the SmartV Report Generator folder through command-line arguments. An example of running the script can be seen below.

```bash
# You may need to replace these paths with your own paths
python3 ./run.py -v /PATH_TO_VRUST/target/debug/vrust -o /PATH_TO_VRUST/SmartV_Report_Generator \
 -s /PATH_TO_PROJECTS/metaplex-program-library -c mpl-metaplex -r /PATH_TO_REPORTS
```

NOTE: Since the target project directory is mounted into the docker, so you must provide the root directory
of the target project to the `-s` flag.

The run.py script also can receive several command-line arguments:
| Flag | Required | Description |
| ----------- | ----------- | ----------- |
| -o | Yes | Location to the SmartV Report Generator folder |
| -v | Yes | Location to the vrust build target |
| -c | No | List of programs (as strings) to run the script on (default is to run all) |
| -r | No | Location to store the .pdf reports (default is ./reports) |
| -s | No | Location of the programs (default is ./) |

## Docker support
The automation script can also be ran in a Docker environment. The `Dockerfile` is found in the root directory of the GitHub repo. 

In order to use it, switch to the root directory of this repo first. For you target project, say its directory is /path,
you can run the below command to analyze it:
```bash
./run.sh /path -c crate_name
```
"crate_name" is the Rust project name that you need to analyze. "/projects" is fixed because we will map your target project
directory to this one. If you want to analyze projects that we have collected (e.g., jet), run the below command:
```bash
./run.sh $(pwd)/rwp -c jet
```

The generated PDF report is under the /home/reports/ folder.

### Docker in Docker

If you want to try this inside a docker container, because by default a docker container cannot create its
child docker container (there are workarounds to it but it's not recommended). So you need to create your
docker container in a different way. For example, if you want to start a ubuntu container, try the below command,
```bash
docker run -v /var/run/docker.sock:/var/run/docker.sock --name test -it ubuntu /bin/bash
```

For running docker again:
```bash
docker start CONTAINER_HASH
docker exec -it CONTAINER_HASH /bin/sh

```

In this way, you are actually creating a sibling docker container which can be accessed from
the current one.

Then run the below command to set up docker and git:
```bash
apt update

# The below commands for setting up docker are from its official document
# https://docs.docker.com/engine/install/ubuntu/
apt install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

apt update
apt install git
apt install docker-ce docker-ce-cli containerd.io 
```

Then you should be able to clone vrust and the target Rust project you want to analyze. For example
```bash
# under /home directory in HOST machine,
# To clone vrust, you need to create ssh key and upload it to your account
git clone git@github.com:parasol-aser/vrust.git

# Note: If there is previous build, you can run `docker rmi -f mirav:latest` to remove the cache.
cd vrust
./run.sh /home/quarry -s /projects -c crate_name

# Note: ./run.sh /home/quarry(HOST) -s /projects -c crate_name(quarry_registry, for example, in quarry repo )
```

NOTE: the path /home/quarry here are the target project repo you cloned on your *HOST* machine.
This is because the two containers are siblings, so both the target project folder needs to
be mounted from the host machine to the mirav docker container.
For example, on you MacOS, run the below command to clone quarry:
```bash
# under /home
git clone https://github.com/SunnyAggregator/quarry.git
```


## diff.py script
A python3 tool (diff.py) is available to run in finding and generating an e-mail containing any differences in .pdf files from two separate directories. It requires >=Python3.6 and a PyPDF2 package installed from pip.

To run the script, you must provide the sender e-mail and its password, the recipient emails, and the two set of directories through command-line arguments. An example of running the script can be seen below.

```python3 diff.py -email o2vrust@gmail.com -pass [redacted] -recipients o2@lists.tamu.edu -prev /yesterdays_reports/ -curr /todays_reports/```

The diff.py script is configured and tested to send through Gmail, but can handle other platforms as long as it is supported by SMTP.

The diff.py script also can receive several command-line arguments:
| Flag | Required | Description |
| ----------- | ----------- | ----------- |
| -email | Yes | Email of the sender |
| -recipients | Yes | Emails of the receiver |
| -pass | Yes | Password to sender email |
| -prev | Yes | Directory path of previous .pdf reports |
| -curr | Yes | Directory path of current .pdf reports |
| -s | No | Email server for SMTP (default is smtp.gmail.com)|
| -p | No | Email server port for SMTP (default is 587) |
