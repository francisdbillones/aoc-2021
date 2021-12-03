from numpy import *
from operator import *
m=map
i=int
a=arange
q=len
p=[*filter(bool,open("i").read().splitlines())]
d=array([[*m(i,l)]for l in p])
g=0+(mean(d,axis=0)>=0.5)
print(i("".join(m(str,g)),2)*i("".join(m(str,1^g)),2))
def s(b,i,c):
    f=b[argwhere(b.T[i]==c(mean(b.T[i]),0.5)).flatten()]
    return f[0]if q(f)==1 else s(f,i+1,c)
print(sum(2**a(q(d[0])-1,-1,-1)*s(d,0,ge))*sum(2**a(q(d[0])-1,-1,-1)*s(d,0,lt)))