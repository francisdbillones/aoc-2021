def main():
    with open("input.txt") as f:
        data = [i for i in f.read().splitlines()]

    # seperate coordinates and fold data into seperate lists
    for i, val in enumerate(data):
        if val == '':
            coordinates = data[:i]
            folds = data[i+1:]
    
    coordinates = [[int(j) for j in i.split(',')] for i in coordinates]

    # for each fold, translate coordinates using formula: num - ((num - fold_num) * 2)
    for fold in folds:
        axis = fold[11]
        fold_num = int(fold[13:])

        if axis == 'y':
            for coordinate in coordinates:
                y = coordinate[1]
                if ((y - fold_num) * 2) >= 0:
                    coordinate[1] = y - ((y - fold_num) * 2)

        if axis == 'x':
            for coordinate in coordinates:
                x = coordinate[0]
                if ((x - fold_num) * 2) >= 0:
                    coordinate[0] = x - ((x - fold_num) * 2)
        
    # get max range for x and y coords
    max_x, max_y = 0, 0
    for coordinate in coordinates:
        if coordinate[0] > max_x:
            max_x = coordinate[0]
        if coordinate[1] > max_y:
            max_y = coordinate[1]

    # make an empty graph
    graph = [[' ' for x in range(max_x + 1)] for y in range(max_y + 1)]

    # append coordinates to the graph with '#'
    for coordinate in coordinates:
        x, y = coordinate[0], coordinate[1]
        if graph[y][x] == ' ':
            graph[y][x] = '#'

    # print the graph
    for line in graph:
        print(''.join(line))


if __name__ == '__main__':
    main()
