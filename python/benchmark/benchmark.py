# %%
from fastwarc.warc import ArchiveIterator
import plsfix
import ftfy
import time
import itertools
import os

num_samples = 1000


script_path = os.path.dirname(os.path.realpath(__file__))


def benchmark(fixer):
    start = time.time()
    for record in itertools.islice(
        ArchiveIterator(open(script_path + "/crawl.warc", "rb")), num_samples
    ):
        byts: bytes = record.reader.read()
        fixer(byts.decode("utf-8", errors="ignore"))
        # plsfix.fix_text(byts.decode("utf-8", errors="ignore"))
        # ftfy.fix_text(byts.decode("utf-8", errors="ignore"))

    end = time.time()

    time_taken = end - start
    records_per_second = num_samples / time_taken

    print(
        f"{fixer.__name__} took {time_taken} seconds ({records_per_second} records per second)"
    )


benchmark(plsfix.fix_text)
benchmark(ftfy.fix_text)
