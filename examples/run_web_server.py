from datetime import datetime
import json
import time
from pyo3_demo import WebServer


def html_callback() -> str:
    current_time = datetime.now().strftime("%H:%M:%S")
    return f"<h1>Hello from Python!</h1>\nThe current time is: <b>{current_time}</b>"


def get_json_callback() -> str:
    json_data = {"current_time": datetime.now().strftime("%H:%M:%S")}
    return json.dumps(json_data)


def post_json_callback(body) -> str:
    json_data = json.loads(body)
    json_data["current_time"] = datetime.now().strftime("%H:%M:%S")
    return json.dumps(json_data)


def main():
    print("Running web server...")
    web_server = WebServer(html_callback, get_json_callback, post_json_callback)
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
