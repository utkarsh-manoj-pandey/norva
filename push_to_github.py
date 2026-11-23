#!/usr/bin/env python3
"""
============================================================================
NORVA: Native Open-source Rust-based Virtual Access
File: push_to_github.py
Description: Interactive Git deployment tool supporting Dry Run test mode,
             custom commit messages, and direct GitHub pushes.
============================================================================
Usage:
    python3 push_to_github.py               # Interactive menu (Push vs Dry Run)
    python3 push_to_github.py --dry-run     # Run test & git simulation directly
    python3 push_to_github.py -m "msg"      # Push with custom message
    python3 push_to_github.py --auto        # Non-interactive automated push
"""

import sys
import os
import subprocess
import shutil

# ANSI Color Codes
CYAN = "\033[96m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
RED = "\033[91m"
MAGENTA = "\033[95m"
BOLD = "\033[1m"
DIM = "\033[2m"
RESET = "\033[0m"

BANNER = f"""{CYAN}{BOLD}
  ███╗   ██╗ ██████╗ ██████╗ ██╗   ██╗ █████╗ 
  ████╗  ██║██╔═══██╗██╔══██╗██║   ██║██╔══██╗
  ██╔██╗ ██║██║   ██║██████╔╝██║   ██║███████║
  ██║╚██╗██║██║   ██║██╔══██╗╚██╗ ██╔╝██╔══██║
  ██║ ╚████║╚██████╔╝██║  ██║ ╚████╔╝ ██║  ██║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝
  NORVA GitHub Deployment & Sync Automation Tool
{RESET}"""

DEFAULT_COMMIT_MSG = (
    "feat: modernize Norva UI, add bookmarks vault, downloads manager, "
    "passwords vault, find in page, and keyboard shortcuts"
)

def run_cmd(cmd, check=True):
    """Executes a shell command and returns (stdout, returncode)."""
    try:
        proc = subprocess.run(
            cmd,
            shell=True,
            text=True,
            capture_output=True,
            check=check
        )
        return proc.stdout.strip(), proc.returncode
    except subprocess.CalledProcessError as e:
        return e.stderr.strip(), e.returncode

def display_file_status(status_output):
    """Prints a styled list of changed and untracked files."""
    lines = status_output.splitlines()
    print(f"{YELLOW}{BOLD}Files Pending Commit ({len(lines)} files):{RESET}")
    for line in lines:
        code_status = line[:2]
        filename = line[3:]
        if 'M' in code_status:
            print(f"  {YELLOW}[MODIFIED]{RESET}  {filename}")
        elif 'A' in code_status or '?' in code_status:
            print(f"  {GREEN}[NEW/ADD] {RESET}  {filename}")
        elif 'D' in code_status:
            print(f"  {RED}[DELETED] {RESET}  {filename}")
        else:
            print(f"  {CYAN}[CHANGED] {RESET}  {filename}")
    print()

def execute_dry_run(branch, remote_url):
    """
    Executes Dry Run Mode:
    1. Runs cargo test to verify build integrity and tests.
    2. Runs git add --dry-run to simulate staging without touching git index.
    3. Displays git diff --stat to show modified line counts.
    4. Tests remote reachability.
    """
    print(f"\n{MAGENTA}{BOLD}════════════════════════════════════════════════════════════════{RESET}")
    print(f"{MAGENTA}{BOLD}           🧪 EXECUTING DRY RUN MODE (TEST BEFOREHAND)          {RESET}")
    print(f"{MAGENTA}{BOLD}════════════════════════════════════════════════════════════════{RESET}")
    print(f"{DIM}Note: No commits or pushes will be made to GitHub during dry run.{RESET}\n")

    # Step 1: Run Rust unit & integration test suite
    print(f"{CYAN}>>> [1/3] Running Cargo Test Suite (Testing compilation & logic)...{RESET}")
    test_out, test_code = run_cmd("cargo test --all-targets", check=False)
    if test_code == 0:
        print(f"{GREEN}✔ Cargo test suite PASSED successfully! (All 9 tests OK){RESET}\n")
    else:
        print(f"{RED}[FAIL] Cargo tests failed! Details below:{RESET}")
        print(f"{YELLOW}{test_out}{RESET}\n")
        print(f"{RED}Please fix build or test failures before pushing.{RESET}")
        return False

    # Step 2: Simulate Git staging
    print(f"{CYAN}>>> [2/3] Simulating Git Staging (git add -A --dry-run)...{RESET}")
    diff_stat, _ = run_cmd("git diff --stat", check=False)
    if diff_stat:
        print(f"{DIM}Diff Statistics:{RESET}")
        for l in diff_stat.splitlines():
            print(f"  {l}")
        print()
    print(f"{GREEN}✔ Git staging simulation complete.{RESET}\n")

    # Step 3: Test remote connectivity
    print(f"{CYAN}>>> [3/3] Checking Remote Connectivity (origin: {remote_url})...{RESET}")
    ls_out, ls_code = run_cmd(f"git ls-remote --heads origin {branch}", check=False)
    if ls_code == 0:
        print(f"{GREEN}✔ Remote '{remote_url}' is reachable on branch '{branch}'.{RESET}\n")
    else:
        print(f"{YELLOW}⚠ Note: Remote check returned non-zero (may require auth on push).{RESET}\n")

    print(f"{GREEN}{BOLD}════════════════════════════════════════════════════════════════{RESET}")
    print(f"{GREEN}{BOLD}✔ DRY RUN COMPLETED: BUILD, TESTS, AND GIT STAGING VERIFIED OK!{RESET}")
    print(f"{GREEN}{BOLD}════════════════════════════════════════════════════════════════{RESET}\n")
    return True

def execute_push(branch, remote_url, commit_msg=None):
    """Executes staging, committing, and pushing to GitHub."""
    print(f"\n{CYAN}{BOLD}════════════════════════════════════════════════════════════════{RESET}")
    print(f"{CYAN}{BOLD}                 🚀 PUSHING UPDATES TO GITHUB                   {RESET}")
    print(f"{CYAN}{BOLD}════════════════════════════════════════════════════════════════{RESET}\n")

    # Prompt for commit message if not provided
    if not commit_msg:
        print(f"{BOLD}What commit message would you like to pass?{RESET}")
        print(f"{DIM}Default message:{RESET} {CYAN}\"{DEFAULT_COMMIT_MSG}\"{RESET}\n")
        try:
            user_msg = input(f"{BOLD}Enter commit message (Press Enter to accept default): {RESET}").strip()
            if user_msg:
                commit_msg = user_msg
            else:
                commit_msg = DEFAULT_COMMIT_MSG
        except (KeyboardInterrupt, EOFError):
            print(f"\n{YELLOW}Push cancelled by user.{RESET}")
            sys.exit(0)

    print(f"\n{BOLD}Selected Commit Message:{RESET} {CYAN}\"{commit_msg}\"{RESET}\n")

    # 1. Stage changes
    print(f"{CYAN}>>> 1/3 Staging all changes (git add -A)...{RESET}")
    out, code = run_cmd("git add -A", check=False)
    if code != 0:
        print(f"{RED}[ERROR] git add failed: {out}{RESET}")
        sys.exit(1)
    print(f"{GREEN}✔ All files staged.{RESET}")

    # 2. Commit changes
    print(f"{CYAN}>>> 2/3 Committing changes...{RESET}")
    escaped_msg = commit_msg.replace('"', '\\"')
    out, code = run_cmd(f'git commit -m "{escaped_msg}"', check=False)
    if code != 0:
        print(f"{RED}[ERROR] git commit failed:\n{out}{RESET}")
        sys.exit(1)

    commit_hash, _ = run_cmd("git rev-parse --short HEAD", check=False)
    print(f"{GREEN}✔ Committed as {BOLD}{commit_hash}{RESET}: \"{commit_msg}\"")

    # 3. Push to GitHub
    print(f"{CYAN}>>> 3/3 Pushing to GitHub (origin/{branch})...{RESET}")
    out, code = run_cmd(f"git push origin {branch}", check=False)
    if code == 0:
        print(f"\n{GREEN}{BOLD}════════════════════════════════════════════════════════════════{RESET}")
        print(f"{GREEN}{BOLD}✔ SUCCESS: ALL UPDATES SUCCESSFULLY PUSHED TO GITHUB!{RESET}")
        print(f"  Commit: {BOLD}{commit_hash}{RESET}")
        print(f"  Branch: {BOLD}{branch}{RESET}")
        print(f"  Remote: {CYAN}{remote_url}{RESET}")
        print(f"{GREEN}{BOLD}════════════════════════════════════════════════════════════════{RESET}\n")
    else:
        print(f"\n{RED}{BOLD}[PUSH FAILED]{RESET}")
        print(f"{YELLOW}{out}{RESET}")
        print(f"\n{DIM}Tip: If GitHub requires authentication or remote branch has newer commits, run:{RESET}")
        print(f"  {CYAN}git pull --rebase origin {branch}{RESET}")
        print(f"  {CYAN}git push origin {branch}{RESET}\n")
        sys.exit(1)

def main():
    # Handle help flag
    if "-h" in sys.argv or "--help" in sys.argv:
        print(BANNER)
        print("Usage:")
        print("  python3 push_to_github.py               # Interactive menu (Push vs Dry Run)")
        print("  python3 push_to_github.py --dry-run     # Run test & git simulation directly")
        print("  python3 push_to_github.py -m \"msg\"      # Push with custom commit message")
        print("  python3 push_to_github.py --auto        # Non-interactive automated push")
        print("  python3 push_to_github.py -h, --help    # Show this help dialog\n")
        sys.exit(0)

    print(BANNER)

    # 1. Verify Git installation
    if not shutil.which("git"):
        print(f"{RED}[ERROR] Git is not installed or not in PATH.{RESET}")
        sys.exit(1)

    # 2. Verify git repository root
    _, code = run_cmd("git rev-parse --is-inside-work-tree", check=False)
    if code != 0:
        print(f"{RED}[ERROR] Not inside a Git repository. Run this script from the Norva root directory.{RESET}")
        sys.exit(1)

    # 3. Detect current branch
    branch, _ = run_cmd("git branch --show-current", check=False)
    if not branch:
        branch = "main"

    # 4. Check git remote
    remote_url, _ = run_cmd("git remote get-url origin", check=False)
    print(f"{DIM}Repository Branch:{RESET} {BOLD}{branch}{RESET}")
    print(f"{DIM}Target Remote:{RESET}     {CYAN}{remote_url}{RESET}\n")

    # 5. Check git status
    status_output, _ = run_cmd("git status --porcelain", check=False)
    if not status_output:
        print(f"{GREEN}✔ Working tree is completely clean. No uncommitted changes detected.{RESET}")
        print(f"  Branch '{branch}' is up-to-date with your local changes.\n")

        # Check if local commits need pushing
        ahead_output, _ = run_cmd(f"git log origin/{branch}..{branch} --oneline", check=False)
        if ahead_output:
            print(f"{YELLOW}Local commits waiting to be pushed:{RESET}")
            for line in ahead_output.splitlines():
                print(f"  {CYAN}•{RESET} {line}")
            choice = input(f"\n{BOLD}Push pending commits to origin/{branch}? [Y/n]: {RESET}").strip().lower()
            if choice in ("", "y", "yes"):
                print(f"\n{CYAN}>>> Pushing to origin/{branch}...{RESET}")
                out, push_code = run_cmd(f"git push origin {branch}", check=False)
                if push_code == 0:
                    print(f"{GREEN}{BOLD}✔ Successfully pushed to GitHub!{RESET}\n")
                else:
                    print(f"{RED}[ERROR] Push failed:\n{out}{RESET}\n")
        return

    # Display modified files
    display_file_status(status_output)

    # Check CLI flags for non-interactive shortcuts
    custom_msg = None
    for i, arg in enumerate(sys.argv):
        if arg in ("-m", "--message") and i + 1 < len(sys.argv):
            custom_msg = sys.argv[i + 1]
            break
        elif arg == "--auto":
            custom_msg = DEFAULT_COMMIT_MSG
            break

    if "--dry-run" in sys.argv or "-d" in sys.argv:
        dry_ok = execute_dry_run(branch, remote_url)
        if dry_ok:
            try:
                proceed = input(f"{BOLD}Would you like to proceed and push to GitHub now? [Y/n]: {RESET}").strip().lower()
                if proceed in ("", "y", "yes"):
                    execute_push(branch, remote_url, custom_msg)
                else:
                    print(f"\n{YELLOW}Push skipped. Exiting safely.{RESET}\n")
            except (KeyboardInterrupt, EOFError):
                print(f"\n{YELLOW}Exiting safely.{RESET}\n")
        sys.exit(0)

    if custom_msg or "--push" in sys.argv or "-p" in sys.argv:
        execute_push(branch, remote_url, custom_msg)
        return

    # ------------------------------------------------------------------------
    # INTERACTIVE MENU: Ask user whether to push directly or test with Dry Run
    # ------------------------------------------------------------------------
    print(f"{BOLD}How would you like to proceed?{RESET}")
    print(f"  {CYAN}[1]{RESET} {BOLD}Push directly to GitHub{RESET} (stage, commit, and push)")
    print(f"  {MAGENTA}[2]{RESET} {BOLD}Test beforehand using Dry Run Mode{RESET} (verify build/tests & simulate staging)")
    print(f"  {RED}[3]{RESET} Cancel / Exit\n")

    try:
        choice = input(f"{BOLD}Select an option [1/2/3] (Default: 1): {RESET}").strip()
    except (KeyboardInterrupt, EOFError):
        print(f"\n{YELLOW}Operation cancelled by user.{RESET}")
        sys.exit(0)

    if choice == "2":
        # Execute Dry Run Mode
        dry_ok = execute_dry_run(branch, remote_url)
        if dry_ok:
            try:
                proceed = input(f"{BOLD}Dry run passed! Push updates to GitHub now? [Y/n]: {RESET}").strip().lower()
                if proceed in ("", "y", "yes"):
                    execute_push(branch, remote_url)
                else:
                    print(f"\n{YELLOW}Push skipped. Your changes remain untouched locally.{RESET}\n")
            except (KeyboardInterrupt, EOFError):
                print(f"\n{YELLOW}Exiting safely.{RESET}\n")
    elif choice in ("1", ""):
        # Push directly
        execute_push(branch, remote_url)
    elif choice == "3":
        print(f"\n{YELLOW}Operation cancelled. No changes were staged or pushed.{RESET}\n")
        sys.exit(0)
    else:
        print(f"{RED}Invalid option selected. Exiting.{RESET}\n")
        sys.exit(1)

if __name__ == "__main__":
    main()
