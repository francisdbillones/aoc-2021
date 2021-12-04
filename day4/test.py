numbers,*boards=open('input.txt').read().split('\n\n')
boards = [[[*map(int,r.split())] for r in b.split('\n')] for b in boards]
won = set()
for num in map(int, numbers.split(',')):
    for b in set(range(len(boards)))-won:
        for r,row in enumerate(boards[b]):
            for c,cell in enumerate(row):
                if cell == num:
                    boards[b][r][c] = -1
                    if sum(boards[b][r]) == -5 or sum(row[c] for row in boards[b]) == -5:
                        won.add(b)
                        if len(won)==1 or len(won)==len(boards):
                            print('winner', sum(sum(c for c in row if c>0) for row in boards[b])*num)