import threading
import os
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor

PATH = "/home/creator/Documents/Paradox/Github/ArtOf"

Files: list[str] = []
for root, dirs, files in os.walk(PATH):
    for file in files:
        Files.append(os.path.join(root, file))


for file in Files:
    with open(file, 'rb') as f:
        for line in f.readline():
            print(line)

# def read_file(filename: str):
#     with open(filename, 'rb') as f:
#         for line in f.readline():
#             print(line)
    

# with ThreadPoolExecutor(max_workers=os.cpu_count()) as p:
#     p.map(read_file, Files)