from pyo3_demo import sum_as_string

def main():
    a = 5
    b = 20
    result = sum_as_string(a, b)
    print(f"{a} + {b} = {result}")
    assert result == str(a + b)

if __name__ == "__main__":
    main()