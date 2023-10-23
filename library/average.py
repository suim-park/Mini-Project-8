import pandas as pd
import time
import resource

def average(path):
    birth_data = pd.read_csv(path)

    weight_avg = birth_data["weight"].mean()
    return weight_avg

def calculate_time_memory(path):
    # Record the start time
    start_time = time.time()

    # Measure the initial resource usage
    start_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

    # Calculate the average
    average(path)

    # Record the end time
    end_time = time.time()

    # Calculate the elapsed time
    elapsed_time = end_time - start_time

    # Measure the final resource usage
    end_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

    print(f"Python-Elapsed Time: {elapsed_time:.7f} seconds")
    print(f"Python-Memory Usage Change: {end_mem_usage - start_mem_usage} kilobytes")

    return end_mem_usage, elapsed_time
