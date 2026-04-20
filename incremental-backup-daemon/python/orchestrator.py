import os

def run_scan():
    home_dir = os.path.expanduser("~")
    print(f"[PYTHON] Running scan on: {home_dir}")
    os.system(f"bash ../bash/scan.sh {home_dir}")

if __name__ == "__main__":
    run_scan()