from datetime import datetime
import time
from pyo3_demo import WebServer


class MyClass:
    pass


def html_handler():
    current_time = datetime.now().strftime("%H:%M:%S")
    return f"<h1>Hello from Python!</h1>\nThe current time is: <b>{current_time}</b>"


def main():
    print("Running web server...")
    web_server = WebServer(html_handler)
    web_server.start()
    for i in reversed(range(1, 6)):
        print(f"Shutting down in {i} ...")
        time.sleep(1)
    web_server.shutdown()
    for i in reversed(range(1, 3)):
        print(f"Exiting in {i} ...")
        time.sleep(1)


if __name__ == "__main__":
    main()
