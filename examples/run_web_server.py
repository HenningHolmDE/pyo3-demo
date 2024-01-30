from datetime import datetime
from pyo3_demo import run_web_server


class MyClass:
    pass


def html_handler():
    current_time = datetime.now().strftime("%H:%M:%S")
    return f"<h1>Hello from Python!</h1>\nThe current time is: <b>{current_time}</b>"


def main():
    print("Running web server...")
    run_web_server(html_handler)


if __name__ == "__main__":
    main()
