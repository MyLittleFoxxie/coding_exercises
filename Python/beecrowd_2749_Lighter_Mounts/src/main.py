def lighter_mounts():
    import sys
    input = sys.stdin.read
    data = input().splitlines()

    results = []
    i = 0
    while i < len(data):
        if data[i].strip() == '':
            i += 1
            continue
        
        N = int(data[i])
        i += 1
        
        mounts = []
        for j in range(N):
            distance_i, tonality_i = map(int, data[i + j].split())
            mounts.append((distance_i, tonality_i))
        i += N

        mounts.sort()

        follows_property = True
        for k in range(1, N):
            if mounts[k - 1][1] <= mounts[k][1]:
                follows_property = False
                break
        
        if follows_property:
            results.append('S')
        else:
            results.append('N')

    for result in results:
        print(result)

if __name__ == "__main__":
    lighter_mounts()
