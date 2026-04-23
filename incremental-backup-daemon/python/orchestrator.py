import os

def run_pipeline():
    home_dir = os.path.expanduser("~")

    print("[PYTHON] Step 1: Scan files")
    os.system(f"bash ../bash/scan.sh {home_dir}")

    print("[PYTHON] Step 2: Run Rust diff engine")
    os.system("cd ../rust && cargo run")

    print("[PYTHON] Step 3: Copy changed files")
    os.system("bash ../bash/copy.sh")

def run_restore(timestamp, target):
    print("[PYTHON] Running restore...")
    os.system(f"bash ../bash/restore.sh {timestamp} {target}")

if __name__ == "__main__":
    choice = input("Enter 'backup' or 'restore': ")

    if choice == "backup":
        run_pipeline()
    elif choice == "restore":
        ts = input("Enter timestamp: ")
        target = input("Enter restore directory: ")
        run_restore(ts, target)