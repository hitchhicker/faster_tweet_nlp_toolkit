from faster_tweet_nlp_toolkit import parse_text
text = parse_text("this is a @mention")
a = text.tokens[0]
print(a + "b")
print("b" + a)
print(a * 3)
print(a * -1)
print(a * 0)
a += 'c'
print(a)
a *= 3
print(a)

b = text.tokens[1]
print(text)

text.tokens[0] = "debug"
print(text)
for i in a:
    print(i)

l = ['a', 'b', 'c', 'd']
l[0] = 'e'
print(l)
c = l[0]
c = 'debug'
print(l)
