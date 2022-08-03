e,k,l=abs,{},{}
for a,c,b,d in[eval(_.replace('->',','))for _ in open('i')]:
 for i in range(max(e(g:=b-a),e(h:=d-c))+1):
  l[(z:=(x,y))]=l.get((x:=a+i*((g>0)-(g<0)),y:=c+i*((h>0)-(h<0))),0)+1
  if e(g)*e(h)==0:k[z]=k.get(z,0)+1
print(sum(k[q]>1 for q in k),sum(l[q]>1 for q in l))
