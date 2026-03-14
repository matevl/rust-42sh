#!/usr/bin/env python3
import subprocess
import os
import sys
import argparse
import difflib

# Attempt to import yaml, if not, fallback to simple parsing or warn
try:
    import yaml
    HAS_YAML = True
except ImportError:
    HAS_YAML = False
    print("Warning: PyYAML not installed. Using rudimentary parser.")

def parse_yaml(file_path):
    if HAS_YAML:
        with open(file_path, 'r') as f:
            return yaml.safe_load(f)
    else:
        cases = []
        current_case = {}
        with open(file_path, 'r') as f:
            for line in f:
                line = line.strip()
                if line.startswith("- name:"):
                    if current_case:
                        cases.append(current_case)
                    current_case = {'name': line.split(":", 1)[1].strip()}
                elif line.startswith("command:"):
                    current_case['command'] = line.split(":", 1)[1].strip()
            if current_case:
                cases.append(current_case)
        return cases

def compile_project(coverage=False):
    print("Compiling project...")
    env = os.environ.copy()
    if coverage:
        env["RUSTFLAGS"] = "-C instrument-coverage"
    
    cmd = ["cargo", "build"]
    if not coverage: # release mode typically, unless debugging. Default to debug for faster cycle in dev
        pass 
        # cmd.append("--release") # Uncomment for release

    result = subprocess.run(cmd, env=env, capture_output=True, text=True)
    if result.returncode != 0:
        print("Compilation failed:")
        print(result.stderr)
        sys.exit(1)
    print("Compilation successful.")

def run_shell(binary, command):
    try:
        # Run our shell
        # Use -c for command execution
        cmd = [binary, "-c", command]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=5)
        return result.stdout, result.stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "Timeout", -1
    except Exception as e:
        return "", str(e), -1

def run_bash(command):
    try:
        cmd = ["bash", "--posix", "-c", command]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=5)
        return result.stdout, result.stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "Timeout", -1

def compare_outputs(case_name, cmd, my_out, my_err, my_code, ref_out, ref_err, ref_code):
    passed = True
    errors = []

    if my_code != ref_code:
        errors.append(f"Exit code mismatch: Expected {ref_code}, Got {my_code}")
        passed = False
    
    # Filter out some noise if necessary, or strict comparison
    if my_out != ref_out:
        diff = difflib.unified_diff(
            ref_out.splitlines(), 
            my_out.splitlines(), 
            fromfile='bash', 
            tofile='rust-42sh', 
            lineterm=''
        )
        errors.append("Stdout mismatch:\n" + "\n".join(diff))
        passed = False

    # Stderr might differ in wording, but maybe check for empty/non-empty equality
    # For now, strict check but maybe relaxed for "error" messages
    # if my_err != ref_err: ...
    
    return passed, errors

def main():
    parser = argparse.ArgumentParser(description="rust-42sh Test Runner")
    parser.add_argument("--coverage", action="store_true", help="Enable coverage instrumentation")
    parser.add_argument("cases", nargs="?", default="tests/cases.yaml", help="Path to test cases YAML")
    args = parser.parse_args()

    compile_project(args.coverage)

    binary_path = "./target/debug/rust-42sh" # Adjust if handling release/debug paths dynamically

    cases = parse_yaml(args.cases)
    if not cases:
        print("No test cases found.")
        sys.exit(0)

    total = 0
    passed = 0
    
    for case in cases:
        total += 1
        name = case.get('name', 'unnamed')
        cmd = case.get('command', '')
        
        print(f"Running test: {name}...", end=" ")
        
        my_out, my_err, my_code = run_shell(binary_path, cmd)
        ref_out, ref_err, ref_code = run_bash(cmd)
        
        is_pass, errors = compare_outputs(name, cmd, my_out, my_err, my_code, ref_out, ref_err, ref_code)
        
        if is_pass:
            print("OK")
            passed += 1
        else:
            print("FAIL")
            for err in errors:
                print(err)
            print("-" * 40)

    print(f"\nSummary: {passed}/{total} tests passed.")
    
    if args.coverage:
        print("\nCoverage data generated (raw profiles). Use grcov or similar to report.")

    if passed < total:
        sys.exit(1)

if __name__ == "__main__":
    main()
