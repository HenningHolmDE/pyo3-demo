from pyo3_demo import run_web_server

class MyClass:
    pass

def html_handler():
    return "<h1>Hello from Python!</h1>"

def main():
    print("Running axum web server...")
    run_web_server(html_handler)

if __name__ == "__main__":
    main()
