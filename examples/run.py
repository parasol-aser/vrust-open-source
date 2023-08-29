from asyncore import write
from dataclasses import dataclass
import argparse, copy, os, subprocess, time, toml
import sys
from pathlib import Path

#####      CMD ARGS      #####

parser = argparse.ArgumentParser()

parser.add_argument('-o', dest='converter', help='Location to .pdf converter project directory', required=True)
parser.add_argument('-v', dest='vrust', help='Location to vrust build', required=True)
parser.add_argument('-c', dest='programs', help='List of specific programs to run only', default=[], nargs='*')
parser.add_argument('-r', dest='reports', help='Location to store reports', default='./reports')
parser.add_argument('-s', dest='source', help='Location to current working directory of programs', default='./')

args = parser.parse_args()

#####     CONSTANTS     #####

PROGRAMS_LIST = args.programs

CONVERTER_LOC = os.path.abspath(args.converter)
REPORTS_LOC = os.path.abspath(args.reports)
VRUST_BUILD_LOC = os.path.abspath(args.vrust)
CWD_SOURCE_LOC = os.path.abspath(args.source)

XARGO_BUILD_CMD = 'RUSTFLAGS="-Zalways-encode-mir -Zsymbol-mangling-version=v0 -C panic=abort" \
                xargo build'
CAGRO_CLEAN_CMD = ['cargo', 'clean']
CONVERTER_CALL_CMD = ['python3', CONVERTER_LOC + '/vrust.py', '--vrust', CONVERTER_LOC, '--dest', REPORTS_LOC] 

TIMESTAMP_CALL = int(time.time())
OUT_FILENAME = 'stdout_{}.out'.format(TIMESTAMP_CALL)
ERR_FILENAME = 'stderr_{}.out'.format(TIMESTAMP_CALL)

#####   DATA STRUCTURES   #####

@dataclass
class Program:
    '''
    Data structure to hold information about a Solana program.
    '''
    name: str               # Project name from Cargo.toml
    path: str                   # Path to the program
    cargo_path: str             # Path to the program's Cargo.toml file
    xargo_path: str             # Path to the program's Xargo.toml file

##### UTILITY FUNCTIONS #####

def is_valid_program(path):
    '''
    A function that returns a bool on whether a Solana program on a specified string path
    is a valid program. (valid directory, valid src directory, valid lib.rs, and valid Cargo.toml)
    '''
    srcDir = path + '/src'
    libFile = srcDir + '/lib.rs'
    cargoFile = path + '/Cargo.toml'

    return all([os.path.isdir(path), os.path.isdir(srcDir), os.path.isfile(libFile),
                os.path.isfile(cargoFile)])

def remove_cfg():
    """
    Remove cfg statement in Rust code that will result in empty compilation.
    """
    cfg_stmt = '#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]\n'
    all_rs_files = list(Path(CWD_SOURCE_LOC).rglob("*.rs"))
    for rs_file in all_rs_files:
        with open(rs_file, "r") as fin:
            data = fin.readlines()
            for idx, line in enumerate(data):
                if line == cfg_stmt:
                    print(f"Comment out cfg line in file: {rs_file}, line: {idx}")
                    data[idx] = "// " + cfg_stmt
        with open(rs_file, "w") as fout:
            fout.writelines(data) 

def get_available_programs():
    '''
    A function that returns a dictionary of programs to run vrust on and data relating to the programs themselves.
    '''
    programs = {}

    def recursive_lookup(src):
        CargoFile = src + '/Cargo.toml'
        XargoFile = src + '/Xargo.toml'

        if is_valid_program(src):
            originalCargo = toml.load(CargoFile)
            libName = originalCargo['package']['name']
            if 'lib' in originalCargo and 'name' in originalCargo['lib']:
                libName = originalCargo['lib']['name']

            if (not PROGRAMS_LIST) or (PROGRAMS_LIST and libName in PROGRAMS_LIST):
                if libName in programs:
                    programs[libName].append(Program(libName, src, CargoFile, XargoFile))

                else:
                    programs[libName] = [Program(libName, src, CargoFile, XargoFile)]

        else:
            for parent_path in os.listdir(src):
                full_parent_path = os.path.join(src, parent_path)

                if os.path.isdir(full_parent_path):
                    recursive_lookup(full_parent_path)

    recursive_lookup(CWD_SOURCE_LOC)

    return programs

def edit_xargo_file(path):
    '''
    A function that edits a program's Xargo.toml file for specific settings to compile 
    with vrust, given a specific string project path as a parameter.
    '''
    tomlContent = {}
    if os.path.isfile(path):
        tomlContent = toml.load(path)
    originalContent = copy.deepcopy(tomlContent)

    # dependencies.std.features=[]
    if 'dependencies' not in tomlContent:
        tomlContent['dependencies'] = {}
    if 'std' not in tomlContent['dependencies']:
        tomlContent['dependencies']['std'] = {}
    if 'features' not in tomlContent['dependencies']['std']:
        tomlContent['dependencies']['std']['features'] = []

    # target.x86_64-unknown-linux-gnu.dependencies={}
    if 'target' not in tomlContent:
        tomlContent['target'] = {}
    if 'x86_64-unknown-linux-gnu' not in tomlContent['target']:
        tomlContent['target']['x86_64-unknown-linux-gnu'] = {}
    if 'dependencies' not in tomlContent['target']['x86_64-unknown-linux-gnu']:
        tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies'] = {}
    
    # target.x86_64-unknown-linux-gnu.dependencies.term.stage=1
    if 'term' not in tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']:
        tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']['term'] = {}
    tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']['term']['stage'] = 1

    # target.x86_64-unknown-linux-gnu.dependencies.proc_macro.stage=2
    if 'proc_macro' not in tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']:
        tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']['proc_macro'] = {}
    tomlContent['target']['x86_64-unknown-linux-gnu']['dependencies']['proc_macro']['stage'] = 2

    # Output to file
    fd = open(path, 'w')
    toml.dump(tomlContent, fd)
    fd.close()

    return originalContent

def xargo_build(path):
    '''
    Executes a system build command from xargo with specific arguments for vrust on a
    specific string path passed as a parameter.
    '''
    # shell=True to pipe xargo build command -- was getting FileNotFoundError exception
    completedProcess = subprocess.Popen(XARGO_BUILD_CMD,
                        cwd=path, shell=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    for line in completedProcess.stdout:
        sys.stdout.write(line)
    subprocess.run(CAGRO_CLEAN_CMD, cwd=path)

    # fd = open(OUT_FILENAME, 'a')
    # fd.write(completedProcess.stdout)
    # fd.close()

    # fd = open(ERR_FILENAME, 'a')
    # fd.write(completedProcess.stderr)
    # fd.close()

def edit_config_file(path):
    '''
    A function that edits a .cargo/config.toml file to point the Rust compiler to a
    vrust build, given a specific string project path as a parameter.
    '''
    cargoDir = path + '/.cargo'
    configFile = cargoDir + '/config.toml'
    if not os.path.isdir(cargoDir):
        os.mkdir(cargoDir)

    tomlContent = {}
    if os.path.isfile(path):
        tomlContent = toml.load(path)
    originalContent = copy.deepcopy(tomlContent)

    tomlContent['build'] = {'rustc': VRUST_BUILD_LOC}
    
    fd = open(configFile, 'w')
    toml.dump(tomlContent, fd)
    fd.close()

    return originalContent

def convert_json_to_pdf(programData):
    '''
    A function that executes an external Python3 script (SmartV Report Generator) to 
    convert the .json report to a .pdf report, given the Program struct of a project.
    '''
    jsonPath = os.path.abspath(programData.path + '/report.json')
    shellCall = CONVERTER_CALL_CMD + ['--output', displayName + '.pdf', jsonPath]
    shellCallStr = ' '.join(shellCall)
    
    print("Generating report: {}".format(shellCallStr))
    completedProcess = subprocess.run(shellCallStr, check=True, 
                        shell=True, capture_output=True, text=True)

def cleanup(programData, originalXargo, originalConfig):
    '''
    A function that restores a project's workspace after running vrust, given the
    Program struct of the project and the original Xargo.toml and config.toml contents.
    '''
    XargoFile = programData.path + '/Xargo.toml'
    configFile = programData.path + '/.cargo/config.toml'
    # reportFile = programData.path + '/report.json'

    if os.path.isfile(XargoFile):
        fd = open(XargoFile, 'w')
        toml.dump(originalXargo, fd)
        fd.close()

    if os.path.isfile(configFile):
        fd = open(configFile, 'w')
        toml.dump(originalConfig, fd)
        fd.close()

    # if os.path.exists(reportFile):
    #     os.remove(reportFile)

def build_vrust():
    cwd = os.getcwd()
    # path = os.path.join(cwd, "vrust")
    path = cwd
    if not os.path.exists(path):
        print("Cannot find vrust folder.")    
        sys.exit(-1)
    os.chdir(path)
    CMD = ["cargo", "build"]
    print("Build vrust in {}".format(path))
    completedProcess = subprocess.Popen(CMD,
                        cwd=path, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    for line in completedProcess.stdout:
        sys.stdout.write(line)
    os.chdir(cwd)


#####    MAIN SCRIPT    #####

build_vrust()

if not os.path.isdir(REPORTS_LOC):
    os.mkdir(REPORTS_LOC)

remove_cfg()
available_programs = get_available_programs()

for programName, programs in available_programs.items():
    hasDuplicates = len(programs) > 1

    for programData in programs:
        displayName = programData.name
        if hasDuplicates:
            displayName = '.'.join(os.path.normpath(programData.path).split(os.sep))

        originalConfig = {}
        originalXargo = edit_xargo_file(programData.xargo_path)

        try:
            print('Starting vrust analysis on {}...'.format(displayName))
            
            xargo_build(programData.path)

            originalConfig = edit_config_file(programData.path)

            xargo_build(programData.path)

            convert_json_to_pdf(programData)

            print('Completed vrust analysis on {}!'.format(displayName))
            print()
        
        except subprocess.CalledProcessError as e:
            print('Encountered error(s) when doing vrust analysis on {}!'.format(displayName))
            print(e.stderr)
            print()
        
        except Exception as e:
            print('Encountered exception during vrust analysis on {}!'.format(displayName))
            print('Exception: {}'.format(repr(e)))
            print()
            
        finally:
            cleanup(programData, originalXargo, originalConfig)

print('Completed all vrust analysis on provided programs!')
print('Reports from the analysis could be found on {} or the reports folder under the target project'
    .format(os.path.abspath(REPORTS_LOC)))
