from time import sleep
from tqdm import tqdm

for i in tqdm(range(100), desc="progres"):
    sleep(.01)
