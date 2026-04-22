import os

def run_pipeline():
    home_dir = os.path.expanduser("~")

    print("[PYTHON] Step 1: Scan files")
    os.system(f"bash ../bash/scan.sh {home_dir}")

    print("[PYTHON] Step 2: Run Rust diff engine")
    os.system("cd ../rust && cargo run")

    print("[PYTHON] Step 3: Copy changed files")
    os.system("bash ../bash/copy.sh")

if __name__ == "__main__":
    run_pipeline()