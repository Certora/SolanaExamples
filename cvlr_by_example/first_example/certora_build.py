#!/usr/bin/env python3
import argparse
import json
import subprocess
import tempfile
import sys
import os
from pathlib import Path

VERBOSE = False
SCRIPT_DIR = Path(__file__).resolve().parent

def log(msg):
    if VERBOSE:
        print(msg, file=sys.stderr)

def run_command(command, to_stdout=False, env=None):
    """Runs the build command and dumps output to temporary files."""
    log(f"Running: {' '.join(command)}")
    try:
        if to_stdout:
            result = subprocess.run(
                ' '.join(command),
                shell=True,
                text=True,
                cwd=SCRIPT_DIR,
                env=env
            )
            return None, None, result.returncode
        else:
            with tempfile.NamedTemporaryFile(delete=False, mode='w', prefix="certora_build_", suffix='.stdout') as stdout_file, \
                tempfile.NamedTemporaryFile(delete=False, mode='w', prefix="certora_build_", suffix='.stderr') as stderr_file:
                # Compile rust project and redirect stdout and stderr to a temp file
                result = subprocess.run(
                    ' '.join(command),
                    shell=True,
                    stderr=stderr_file,
                    text=True,
                    cwd=SCRIPT_DIR,
                    env=env
                )
                return stdout_file.name, stderr_file.name, result.returncode
    except Exception as e:
        log(f"Error running command': {e}")
        return None, None -1

def main():
    parser = argparse.ArgumentParser(description="Compile rust projects and generate JSON output to be used by Certora Prover.")
    parser.add_argument("--json", action="store_true", help="Dump JSON output to the console.")
    parser.add_argument("--cargo_features", nargs="+", help="Additional features to pass to cargo")
    parser.add_argument("-v", "--verbose", action="store_true", help="Be verbose.")
    parser.add_argument("-l", "--log", action="store_true", help="Show log outputs from cargo build on standard out.")

    args = parser.parse_args()
    global VERBOSE
    VERBOSE = args.verbose

    to_stdout = args.log

    cargo_cmd = ['cargo', 'certora-sbf']

    if args.json:
        cargo_cmd.append("--json")

    if args.cargo_features:
        cargo_cmd.append('--features')
        cargo_cmd.extend(args.cargo_features)

    # Compile rust project and dump the logs to tmp files
    stdout_log, stderr_log, return_code = run_command(cargo_cmd, to_stdout)

    if stdout_log is not None:
        log(f"Temporary log file located at:\n\t{stdout_log}\nand\n\t{stderr_log}")

    # Needed for mutations: if you run _this_ script inside another script, you can check this returncode and decide what to do
    sys.exit(0 if return_code == 0 else 1)

if __name__ == "__main__":
    main()