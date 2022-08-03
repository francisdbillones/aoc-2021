class infinite_range:
    def __init__(self, start=0):
        self.start = start
        self.cur = start

    def __iter__(self):
        return infinite_range_iterator(self.start)


class infinite_range_iterator:
    def __init__(self, start: int):
        self.cur = start

    def __iter__(self):
        return self

    def __next__(self):
        self.cur += 1
        return self.cur
