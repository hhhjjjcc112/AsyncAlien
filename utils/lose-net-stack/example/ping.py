import socket
import sys
import time

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
addr = ('localhost', int(sys.argv[1]))
buf = "this is a ping!".encode('utf-8')
print(addr)
sock.connect(addr)
sock.send(buf)
count = 0

while True:
	time.sleep(1)
	buf = sock.recv(4096)
	if buf:
		print("receive the reply from qemu, the content is: {}".format(buf.decode("utf-8")))
		if buf.decode("utf-8") == "end":
			print("test pass!")
			break
		else:
			# print("send the ping to qemu {}".format(addr))
			sock.sendto("this is {}  ping!".format(count).encode('utf-8'), addr)
			count += 1