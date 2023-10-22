from library.average import average, calculate_time_memory

data_path = "births14.csv"

def test_average():
    result = average("births14.csv")
    expected_result = 7.19816

    assert result == expected_result, "Test has failed."

def test_calculate_time_memory():
    result = calculate_time_memory("births14.csv")
    
    assert result is not None, "Test has failed."

if __name__ == "__main__":
    test_average()
    test_calculate_time_memory()