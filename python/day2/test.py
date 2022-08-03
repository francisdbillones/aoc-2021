from numpy import *
c,m=map(array,zip(*[(c[0],int(m))for c,m in[l.split()for l in open("i").readlines()]]))
print((m*(c=="f")).sum()*(m*cumsum(m*((c=="d")-1*(c=="u")))*(c=="f")).sum())