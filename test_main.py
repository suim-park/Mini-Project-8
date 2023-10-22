from library.average import average, calculate_time_memory

data_path = "births14.csv"

def test_average(path):
    result = average(path)
    expected_result = 7.19816

    assert result == expected_result, "Test has failed."

if __name__ == "__main__":
    test_average(data_path)